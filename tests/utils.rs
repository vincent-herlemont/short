use assert_cmd::Command;
use cli_integration_test::IntegrationTestEnvironment;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const HOME: &'static str = "home";
pub const PROJECT: &'static str = "project";
pub const ENVDIR: &'static str = "private_env";

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PathTestEnvironment {
    LocalCfg,
    GlobalCfg,
    LocalEnvExample,
    GlobalEnvDev,
    GlobalEnvProd,
}

pub struct IntegrationTestEnvironmentWrapper {
    e: IntegrationTestEnvironment,
    paths: HashMap<PathTestEnvironment, PathBuf>,
}

impl IntegrationTestEnvironmentWrapper {
    pub fn init_all<S: AsRef<str>>(label: S) -> Self {
        let mut itew = Self {
            e: IntegrationTestEnvironment::new(label),
            paths: HashMap::new(),
        };

        itew.set_local_cfg();
        itew.set_global_cfg();
        itew.set_local_env_example();
        itew.set_global_env_dev();
        itew.set_global_env_pro();

        itew.e.add_dir(PROJECT);
        itew.e.add_dir(HOME);
        itew.e.add_dir(ENVDIR);
        itew.e.setup();

        return itew;
    }

    pub fn command<C>(&self, crate_name: C) -> Command
    where
        C: AsRef<str>,
    {
        let mut command = self.e.command(crate_name);
        command.current_dir(&self.e.path().join(PROJECT));
        command
    }

    pub fn get_integration_test_environment(&self) -> &IntegrationTestEnvironment {
        &self.e
    }

    pub fn get_rel_path(&self, path: PathTestEnvironment) -> Option<PathBuf> {
        self.paths.get(&path).map(|path| path.clone())
    }

    pub fn get_abs_path(&self, path: PathTestEnvironment) -> Option<PathBuf> {
        self.get_rel_path(path)
            .map(|path| self.e.path().join(&path))
    }

    pub fn set_path<P: AsRef<Path>>(&mut self, setup_path: PathTestEnvironment, path: P) {
        self.paths.insert(setup_path, path.as_ref().to_path_buf());
    }

    pub fn set_local_cfg(&mut self) {
        self.set_path(
            PathTestEnvironment::LocalCfg,
            PathBuf::from(PROJECT).join("short.yml"),
        );
    }

    pub fn set_global_cfg(&mut self) {
        self.set_path(
            PathTestEnvironment::GlobalCfg,
            PathBuf::from(HOME).join(".short/cfg.yml"),
        );
    }

    pub fn set_local_env_example(&mut self) {
        self.set_path(
            PathTestEnvironment::LocalEnvExample,
            PathBuf::from(ENVDIR).join(".example"),
        );
    }

    pub fn set_global_env_dev(&mut self) {
        self.set_path(
            PathTestEnvironment::GlobalEnvDev,
            PathBuf::from(ENVDIR).join(".dev"),
        );
    }

    pub fn set_global_env_pro(&mut self) {
        self.set_path(
            PathTestEnvironment::GlobalEnvProd,
            PathBuf::from(ENVDIR).join(".prod"),
        );
    }
}
