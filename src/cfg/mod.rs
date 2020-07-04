use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::{Context, Result};

pub use error::CfgError;
pub use file::load_local_cfg;
pub use file::FileCfg;
pub use global::GlobalCfg;
pub use local::LocalCfg;
pub use local::LocalSetupCfg;
pub use local::{ArrayVar, ArrayVars, VarCase, VarName, Vars};
pub use setup::Setup;
pub use setup::SetupCfg;
pub use setup::SetupsCfg;

use crate::cfg::file::{load_or_new_global_cfg, new_local_cfg};
use crate::cfg::global::GlobalProjectCfg;

mod error;
mod file;
mod global;
mod local;
mod setup;

#[derive(Debug)]
pub struct Cfg {
    local_cfg: FileCfg<LocalCfg>,
    global_cfg: FileCfg<GlobalCfg>,
}

impl Cfg {
    pub fn load_local(global_dir: PathBuf, local_dir: PathBuf) -> Result<Self> {
        let local_cfg = load_local_cfg(&local_dir).context("fail to load local cfg file")?;

        Cfg::new(global_dir, local_cfg)
    }

    pub fn create_local(global_dir: PathBuf, local_dir: PathBuf) -> Result<Self> {
        let local_cfg = new_local_cfg(&local_dir).context("fail to create local cfg file")?;
        Cfg::new(global_dir, local_cfg)
    }

    pub fn new(global_dir: PathBuf, local_cfg: FileCfg<LocalCfg>) -> Result<Self> {
        let global_cfg =
            load_or_new_global_cfg(&global_dir).context("fail to load global cfg file")?;

        Ok(Self {
            local_cfg,
            global_cfg,
        })
    }

    pub fn save(&self) -> Result<()> {
        self.local_cfg.save()?;
        self.global_cfg.save()?;
        Ok(())
    }

    pub fn add_local_setup_cfg(&mut self, setup: LocalSetupCfg) {
        let local_cfg = self.local_cfg.borrow_mut();
        local_cfg.add_setup(setup);
    }

    pub fn sync_local_to_global(&mut self) -> Result<()> {
        let global_cfg = self.global_cfg.borrow_mut();
        global_cfg
            .sync_local_project(&self.local_cfg)
            .context("fail to sync local project cfg to global")?;
        Ok(())
    }

    /**
     * Local cfg and Global cfg must be synchronised before.
     **/
    pub fn current_project(&self) -> Result<Rc<RefCell<GlobalProjectCfg>>> {
        let local_cfg_file = self.local_cfg.file()?;
        let global_cfg = self.global_cfg.borrow();
        global_cfg
            .get_project_by_file(local_cfg_file)
            .context(format!("fail to get project {:?}", local_cfg_file))
    }

    /**
     * Local cfg and Global cfg must be synchronised before.
     **/
    pub fn current_setups(&self) -> Result<Vec<Setup>> {
        let global_project = self.current_project()?;

        let local_setups = self.local_cfg.borrow().get_setups();

        let local_cfg_path = self.local_cfg.file()?;

        let setups: Vec<_> = local_setups
            .borrow()
            .iter()
            .filter_map(|local_setup| {
                if let Some(global_setup) = global_project
                    .borrow()
                    .get_setup(local_setup.borrow().name())
                {
                    if let Ok(setup) = Setup::new_fill(local_cfg_path, &local_setup, &global_setup)
                    {
                        return Some(setup);
                    }
                }
                None
            })
            .collect();

        Ok(setups)
    }

    pub fn current_setup(&self, name: &String) -> Result<Setup> {
        for setup in self.current_setups()? {
            if setup.name()? == *name {
                return Ok(setup);
            }
        }
        bail!(CfgError::SetupNotFound(name.to_owned()))
    }
}

#[cfg(test)]
mod main_test {
    use std::path::PathBuf;

    use cli_integration_test::IntegrationTestEnvironment;
    use predicates::path::{exists, is_file};
    use predicates::prelude::*;
    use predicates::str::contains;

    use crate::cfg::Cfg;

    pub const HOME: &'static str = "home";
    pub const PROJECT: &'static str = "project";

    pub fn init_env() -> IntegrationTestEnvironment {
        let mut e = IntegrationTestEnvironment::new("cfg");
        e.add_dir(HOME);
        e.add_dir(PROJECT);
        e.setup();
        e
    }

    #[test]
    fn create_cfg() {
        let e = init_env();
        let local_cfg = e.path().unwrap().join(PROJECT).join("short.yaml");
        let global_cfg = e.path().unwrap().join(HOME).join(".short/cfg.yaml");

        let cfg = Cfg::create_local(
            e.path().unwrap().join(HOME),
            e.path().unwrap().join(PROJECT),
        )
        .unwrap();
        assert!(!exists().eval(&local_cfg));
        assert!(!exists().eval(&global_cfg));
        cfg.save().unwrap();
        assert!(exists().eval(&local_cfg));
        assert!(exists().eval(&global_cfg));
    }

    #[test]
    fn create_sync_global_cfg() {
        let mut e = init_env();
        let local_cfg = PathBuf::from(PROJECT).join("short.yaml");
        let global_cfg = PathBuf::from(HOME).join(".short/cfg.yaml");

        let abs_local_cfg = e.path().unwrap().join(&local_cfg);
        let abs_global_cfg = e.path().unwrap().join(&global_cfg);

        e.add_file(
            &local_cfg,
            r#"
setups:
  - name: setup_1
    file: ./run.sh
        "#,
        );
        e.setup();

        let mut cfg = Cfg::load_local(
            e.path().unwrap().join(HOME),
            e.path().unwrap().join(PROJECT),
        )
        .unwrap();
        cfg.sync_local_to_global().unwrap();
        let setups = cfg.current_setups();

        // Check content of setups
        let dbg_setups = format!("{:#?}", setups);
        assert!(contains("setup_1").count(2).eval(dbg_setups.as_str()));

        // Check if global file do not exist before save
        assert!(!is_file().eval(&abs_global_cfg));
        cfg.save().unwrap();
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
        let local_cfg = PathBuf::from(PROJECT).join("short.yaml");
        let global_cfg = PathBuf::from(HOME).join(".short/cfg.yaml");

        let abs_local_cfg = e.path().unwrap().join(&local_cfg);
        let _abs_global_cfg = e.path().unwrap().join(&global_cfg);

        e.add_file(
            &local_cfg,
            r#"
setups:
  - name: setup_1
    file: ./run.sh
  - name: setup_2
    file: ./run.sh
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
        let cfg = Cfg::load_local(
            e.path().unwrap().join(HOME),
            e.path().unwrap().join(PROJECT),
        )
        .unwrap();
        let setup_1 = cfg.current_setup(&"setup_1".into()).unwrap();
        setup_1
            .global_setup()
            .unwrap()
            .borrow_mut()
            .set_private_env_dir("/private/env/dir2".into())
            .unwrap();

        cfg.save().unwrap();

        // Check the content of global file
        let global_cfg_str = e.read_file(&global_cfg);
        assert!(contains("/private/env/dir2")
            .count(1)
            .eval(global_cfg_str.as_str()));

        // Rename setup_1 to setup_3
        setup_1.rename(&"setup_3".into()).unwrap();
        cfg.save().unwrap();

        let global_cfg_str = e.read_file(&global_cfg);
        assert!(contains("setup_1").count(0).eval(global_cfg_str.as_str()));
        assert!(contains("setup_3").count(1).eval(global_cfg_str.as_str()));

        let local_cfg_str = e.read_file(&local_cfg);
        assert!(contains("setup_1").count(0).eval(local_cfg_str.as_str()));
        assert!(contains("setup_3").count(1).eval(local_cfg_str.as_str()));
    }

    #[test]
    fn load_envs_public_and_private() {
        const ENVDIR: &'static str = "private_env";
        let mut e = init_env();
        let local_cfg = PathBuf::from(PROJECT).join("short.yaml");
        let global_cfg = PathBuf::from(HOME).join(".short/cfg.yaml");
        let env_example = PathBuf::from(PROJECT).join(".example");
        let env_dev = PathBuf::from(ENVDIR).join(".dev");
        let env_prod = PathBuf::from(ENVDIR).join(".prod");

        let abs_local_cfg = e.path().unwrap().join(&local_cfg);
        let _abs_global_cfg = e.path().unwrap().join(&global_cfg);
        let abs_env_example = e.path().unwrap().join(env_example);
        let abs_env_dev = e.path().unwrap().join(env_dev);
        let abs_env_prod = e.path().unwrap().join(env_prod);

        e.add_file(
            &local_cfg,
            r#"
setups:
  - name: setup_1
    file: ./run.sh
    array_vars: {}
        "#,
        );
        e.add_file(
            &abs_env_example,
            r#"
ENV= example
VAR2= value2
VAR3= "example"
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
        private_env_dir: {}
                "#,
                abs_local_cfg.to_string_lossy(),
                e.path().unwrap().join(ENVDIR).to_string_lossy(),
            ),
        );
        e.add_file(
            &abs_env_prod,
            r#"
ENV=prod
"#,
        );
        e.add_file(
            &abs_env_dev,
            r#"
ENV=dev
"#,
        );
        e.setup();

        let cfg = Cfg::load_local(
            e.path().unwrap().join(HOME),
            e.path().unwrap().join(PROJECT),
        )
        .unwrap();
        let setup_1 = cfg.current_setup(&"setup_1".into()).unwrap();
        let env_public = setup_1.envs_public();
        assert!(env_public.iter().count().eq(&1));
        let env_private = setup_1.envs_private();
        assert!(env_private.iter().count().eq(&2));
    }
}
