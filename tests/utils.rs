use assert_cmd::Command;
use cli_integration_test::IntegrationTestEnvironment;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[deprecated]
pub const HOME: &'static str = "home";
#[deprecated]
pub const PROJECT: &'static str = "project";
#[deprecated]
pub const ENVDIR: &'static str = "private_env";

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[deprecated]
pub enum PathTestEnvironment {
    LocalCfg,
    GlobalCfg,
    LocalEnvExample,
    LocalEnvExample2,
    GlobalEnvDev,
    GlobalEnvProd,
    Run,
}

#[deprecated]
pub struct IntegrationTestEnvironmentWrapper {
    e: Rc<RefCell<IntegrationTestEnvironment>>,
    paths: HashMap<PathTestEnvironment, PathBuf>,
}

impl IntegrationTestEnvironmentWrapper {
    #[deprecated]
    pub fn init_all<S: AsRef<str>>(label: S) -> Self {
        let mut itew = Self {
            e: Rc::new(RefCell::new(IntegrationTestEnvironment::new(label))),
            paths: HashMap::new(),
        };

        itew.set_local_cfg();
        itew.set_global_cfg();
        itew.set_local_env_example();
        itew.set_local_env_example2();
        itew.set_global_env_dev();
        itew.set_global_env_pro();
        itew.set_run();

        let e = itew.e();
        let mut e = e.borrow_mut();

        e.add_dir(PROJECT);
        e.add_dir(HOME);
        e.add_dir(ENVDIR);
        e.setup();

        return itew;
    }

    #[deprecated]
    pub fn command<C>(&self, crate_name: C) -> Command
    where
        C: AsRef<str>,
    {
        let e = self.e();
        let e = e.borrow_mut();
        let mut command = e.command(crate_name);
        command.current_dir(&e.path().join(PROJECT));
        command.env("HOME", &e.path().join(HOME));
        command
    }

    #[deprecated]
    pub fn e(&self) -> Rc<RefCell<IntegrationTestEnvironment>> {
        Rc::clone(&self.e)
    }

    #[deprecated]
    pub fn get_rel_path(&self, path: PathTestEnvironment) -> Option<PathBuf> {
        self.paths.get(&path).map(|path| path.clone())
    }

    #[deprecated]
    pub fn get_abs_path(&self, path: PathTestEnvironment) -> Option<PathBuf> {
        let e = self.e();
        let e = e.borrow_mut();
        self.get_rel_path(path).map(|path| e.path().join(&path))
    }

    #[deprecated]
    pub fn set_path<P: AsRef<Path>>(&mut self, setup_path: PathTestEnvironment, path: P) {
        self.paths.insert(setup_path, path.as_ref().to_path_buf());
    }

    #[deprecated]
    pub fn set_local_cfg(&mut self) {
        self.set_path(
            PathTestEnvironment::LocalCfg,
            PathBuf::from(PROJECT).join("short.yml"),
        );
    }

    #[deprecated]
    pub fn set_global_cfg(&mut self) {
        self.set_path(
            PathTestEnvironment::GlobalCfg,
            PathBuf::from(HOME).join(".short/cfg.yml"),
        );
    }

    #[deprecated]
    pub fn set_local_env_example(&mut self) {
        self.set_path(
            PathTestEnvironment::LocalEnvExample,
            PathBuf::from(PROJECT).join(".example"),
        );
    }

    #[deprecated]
    pub fn set_local_env_example2(&mut self) {
        self.set_path(
            PathTestEnvironment::LocalEnvExample2,
            PathBuf::from(PROJECT).join(".example2"),
        );
    }

    #[deprecated]
    pub fn set_global_env_dev(&mut self) {
        self.set_path(
            PathTestEnvironment::GlobalEnvDev,
            PathBuf::from(ENVDIR).join(".dev"),
        );
    }

    #[deprecated]
    pub fn set_global_env_pro(&mut self) {
        self.set_path(
            PathTestEnvironment::GlobalEnvProd,
            PathBuf::from(ENVDIR).join(".prod"),
        );
    }

    #[deprecated]
    pub fn set_run(&mut self) {
        self.set_path(
            PathTestEnvironment::Run,
            PathBuf::from(PROJECT).join("run.sh"),
        );
    }
}
