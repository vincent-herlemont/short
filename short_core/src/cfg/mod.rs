use std::cell::{Ref, RefCell};
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};

use anyhow::{Context, Result};

pub use env::EnvPathCfg;
pub use env::EnvPathsCfg;
pub use global::GlobalCfg;
pub use local::LocalCfg;
pub use local::LocalSetupCfg;
pub use local::LocalSetupProviderCfg;
pub use project::ProjectCfg;
pub use setup::SetupCfg;
pub use setup::SetupsCfg;

use crate::cfg::file::{load_or_new_global_cfg, load_or_new_local_cfg, FileCfg};
use crate::cfg::global::{GlobalProjectCfg, GlobalProjectSetupCfg, GLOCAL_FILE_NAME};
use crate::cfg::setup::Setup;

mod env;
mod file;
mod global;
mod local;
mod project;
mod setup;

#[derive(Debug)]
pub struct Cfg {
    current_setup: Setup,
    local_cfg: FileCfg<LocalCfg>,
    global_cfg: FileCfg<GlobalCfg>,
}

impl Cfg {
    pub fn load(global_dir: PathBuf, local_dir: PathBuf) -> Result<Self> {
        let local_cfg = load_or_new_local_cfg(&local_dir).context("fail to load local cfg file")?;

        let global_cfg =
            load_or_new_global_cfg(&global_dir).context("fail to load global cfg file")?;

        Ok(Self {
            current_setup: Setup::new(),
            local_cfg,
            global_cfg,
        })
    }

    pub fn save(&self) -> Result<()> {
        self.local_cfg.save()?;
        self.global_cfg.save()?;
        Ok(())
    }

    pub fn local_setups(&mut self) -> Result<Vec<Setup>> {
        let mut out = vec![];
        if let Some(local_path) = self.local_cfg.path() {
            let mut global_setups = vec![];
            let mut global_project = if let Some(global_project) =
                self.global_cfg.borrow().get_project_by_file(local_path)
            {
                // Load global setups
                global_setups.append(&mut vec![global_project.borrow().get_setups()]);
                global_project
            } else {
                // Create empty global project
                let global_project = GlobalProjectCfg::new(local_path)?;
                self.global_cfg.borrow_mut().add_project(global_project);
                self.global_cfg
                    .borrow()
                    .get_project_by_file(local_path)
                    .unwrap()
            };

            // Sync local setup to global setup
            let local_setups = self.local_cfg.borrow().get_setups();
            for local_setup in local_setups.borrow().iter() {
                let global_setup = GlobalProjectSetupCfg::from(&*local_setup.borrow());
                global_project.borrow_mut().add_setup(global_setup);
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use predicates::path::{exists, is_file};
    use predicates::prelude::Predicate;
    use predicates::str::contains;

    use short_utils::integration_test::environment::IntegrationTestEnvironment;

    use crate::cfg::Cfg;

    const HOME: &'static str = "home";
    const PROJECT: &'static str = "project";

    fn init_env() -> IntegrationTestEnvironment {
        let mut e = IntegrationTestEnvironment::new("cfg");
        e.add_dir(HOME);
        e.add_dir(PROJECT);
        e.setup();
        e
    }

    #[test]
    fn new_and_save_cfg() {
        let e = init_env();
        let local_cfg = e.path().join(PROJECT).join("short.yml");
        let global_cfg = e.path().join(HOME).join(".short/cfg.yml");

        let cfg = Cfg::load(e.path().join(HOME), e.path().join(PROJECT)).unwrap();
        assert!(!exists().eval(&local_cfg));
        assert!(!exists().eval(&global_cfg));
        cfg.save().unwrap();
        assert!(exists().eval(&local_cfg));
        assert!(exists().eval(&global_cfg));
    }

    #[test]
    fn sync_global_cfg() {
        let mut e = init_env();
        let local_cfg = PathBuf::from(PROJECT).join("short.yml");
        let global_cfg = PathBuf::from(HOME).join(".short/cfg.yml");

        let abs_local_cfg = e.path().join(&local_cfg);
        let abs_global_cfg = e.path().join(&global_cfg);

        e.add_file(
            &local_cfg,
            r#"
setups:
  - name: setup_1
    provider:
      name: cloudformation
      template: ./template_1.yaml
        "#,
        );
        e.setup();

        let mut cfg = Cfg::load(e.path().join(HOME), e.path().join(PROJECT)).unwrap();
        cfg.local_setups();
        assert!(!is_file().eval(&abs_global_cfg));
        cfg.save();
        assert!(is_file().eval(&abs_global_cfg));

        // Check is abs path of local file is present
        let global_cfg_str = e.read_file(&global_cfg);
        assert!(
            contains(format!("{}", &abs_local_cfg.to_string_lossy())).eval(global_cfg_str.as_str())
        );
        assert!(contains("setup_1").eval(global_cfg_str.as_str()));
        println!("{}", e.read_file(&global_cfg));
    }

    #[test]
    fn load_and_mutate() {
        let mut e = init_env();
        let local_cfg = PathBuf::from(PROJECT).join("short.yml");
        let global_cfg = PathBuf::from(HOME).join(".short/cfg.yml");

        let abs_local_cfg = e.path().join(&local_cfg);
        let abs_global_cfg = e.path().join(&global_cfg);

        e.add_file(
            &local_cfg,
            r#"
setups:
  - name: setup_1
    provider:
      name: cloudformation
      template: ./template_1.yaml
        "#,
        );
        e.add_file(
            &global_cfg,
            format!(
                r#"
projects:
  - file: '{}'
    setups:
      - name: setup_1
        private_env_dir: /private/env/dir
                "#,
                abs_local_cfg.to_string_lossy()
            ),
        );

        e.setup();
        dbg!(e.tree());
        let mut cfg = Cfg::load(e.path().join(HOME), e.path().join(PROJECT)).unwrap();
        let setup = cfg.local_setups();
        dbg!(setup);

        dbg!(e.tree());
    }
}
