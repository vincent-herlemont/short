use std::cell::{Ref, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Weak;

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

use crate::cfg::file::{FileCfg, load_or_new_global_cfg, load_or_new_local_cfg};
use crate::cfg::global::GLOCAL_FILE_NAME;
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
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use predicates::path::exists;
    use predicates::prelude::Predicate;

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
    fn load_mutate_and_save_cfg() {
        let mut e = init_env();
        let local_cfg = PathBuf::from(PROJECT).join("short.yml");
        let global_cfg = PathBuf::from(HOME).join(".short/cfg.yml");

        e.add_file(
            &local_cfg,
            r#"
setups:
  - name: 'setup_1'
    provider:
      name: cloudformation
      template: ./template_1.yaml
        "#,
        );
        e.setup();
        dbg!(e.tree());

        let local_cfg = e.path().join(&local_cfg);
        let global_cfg = e.path().join(global_cfg);

        let cfg = Cfg::load(e.path().join(HOME), e.path().join(PROJECT)).unwrap();
        dbg!(cfg);
    }
}
