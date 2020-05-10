use crate::cfg::file::{load_or_new_global_cfg, load_or_new_local_cfg, FileCfg};
use crate::cfg::global::{GlobalProjectCfg, GlobalProjectSetupCfg, GLOCAL_FILE_NAME};
use crate::cfg::setup::Setup;
use anyhow::{Context, Result};
use std::cell::{Ref, RefCell};
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};

pub use env::EnvPathCfg;
pub use env::EnvPathsCfg;
pub use global::GlobalCfg;
pub use local::LocalCfg;
pub use local::LocalSetupCfg;
pub use local::LocalSetupProviderCfg;
pub use project::ProjectCfg;
pub use setup::SetupCfg;
pub use setup::SetupsCfg;

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
        let global_cfg = self.global_cfg.borrow_mut();

        let global_project = global_cfg
            .sync_local_project(&self.local_cfg)
            .context("fail to sync local project cfg to global")?;

        let local_setups = self.local_cfg.borrow().get_setups();

        let setups: Vec<_> = local_setups
            .borrow()
            .iter()
            .filter_map(|local_setup| {
                if let Some(global_setup) = global_project
                    .borrow()
                    .get_setup(local_setup.borrow().name())
                {
                    if let Ok(setup) =
                        Setup::new_fill(Rc::downgrade(local_setup), Rc::downgrade(&global_setup))
                    {
                        return Some(setup);
                    }
                }
                None
            })
            .collect();

        Ok(setups)
    }

    pub fn local_setup(&mut self, name: &String) -> Result<Option<Setup>> {
        for setup in self.local_setups()? {
            if setup.name()? == *name {
                return Ok(Some(setup));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use predicates::path::{exists, is_file};
    use predicates::prelude::Predicate;
    use predicates::str::contains;

    use short_utils::integration_test::environment::IntegrationTestEnvironment;

    use crate::cfg::{Cfg, EnvPathCfg};

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
    fn create_sync_global_cfg() {
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
        let setups = cfg.local_setups();

        // Check content of setups
        let dbg_setups = format!("{:#?}", setups);
        assert!(contains("setup_1").count(2).eval(dbg_setups.as_str()));
        assert!(contains("./template_1.yaml")
            .count(1)
            .eval(dbg_setups.as_str()));

        // Check if global file do not exist before save
        assert!(!is_file().eval(&abs_global_cfg));
        cfg.save();
        // Check if global file do not exist after save
        assert!(is_file().eval(&abs_global_cfg));

        // Check the content of global file
        let global_cfg_str = e.read_file(&global_cfg);
        assert!(
            contains(format!("{}", &abs_local_cfg.to_string_lossy())).eval(global_cfg_str.as_str())
        );
        assert!(contains("setup_1").eval(global_cfg_str.as_str()));
    }

    #[test]
    fn load_and_mutate_cfg() {
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
  - name: setup_2
    provider:
      name: cloudformation
      template: ./template_2.yaml
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
        let mut cfg = Cfg::load(e.path().join(HOME), e.path().join(PROJECT)).unwrap();
        let mut setup_1 = cfg.local_setup(&"setup_1".into()).unwrap().unwrap();
        setup_1
            .global_setup()
            .unwrap()
            .borrow_mut()
            .set_env_path_op(Some("/private/env/dir2".into()));

        cfg.save();

        // Check the content of global file
        let global_cfg_str = e.read_file(&global_cfg);
        assert!(contains("/private/env/dir2")
            .count(1)
            .eval(global_cfg_str.as_str()));

        // Rename setup_1 to setup_3
        setup_1.rename(&"setup_3".into());
        cfg.save();

        let global_cfg_str = e.read_file(&global_cfg);
        assert!(contains("setup_1").count(0).eval(global_cfg_str.as_str()));
        assert!(contains("setup_3").count(1).eval(global_cfg_str.as_str()));

        let local_cfg_str = e.read_file(&local_cfg);
        assert!(contains("setup_1").count(0).eval(local_cfg_str.as_str()));
        assert!(contains("setup_3").count(1).eval(local_cfg_str.as_str()));
    }
}
