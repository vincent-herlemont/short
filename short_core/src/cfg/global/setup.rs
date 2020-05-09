use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::cfg::{EnvPathCfg, SetupCfg};

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalProjectSetupCfg {
    name: String,
    private_env_dir: Option<PathBuf>,
}

impl GlobalProjectSetupCfg {
    pub fn new(name: String) -> Self {
        Self {
            name,
            private_env_dir: None,
        }
    }
}

impl SetupCfg for GlobalProjectSetupCfg {
    fn name(&self) -> String {
        self.name.to_owned()
    }
}

impl EnvPathCfg for GlobalProjectSetupCfg {
    fn env_path_op(&self) -> Option<&PathBuf> {
        self.private_env_dir.as_ref()
    }

    fn set_env_path_op(&mut self, dir: Option<PathBuf>) {
        self.private_env_dir = dir
    }
}
