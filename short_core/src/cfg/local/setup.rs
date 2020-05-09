use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::cfg::{EnvPathCfg, LocalSetupProviderCfg};
use crate::cfg::setup::SetupCfg;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalSetupCfg {
    name: String,

    public_env_directory: Option<PathBuf>,

    provider: LocalSetupProviderCfg,
}

impl LocalSetupCfg {
    pub fn new(name: String, provider: LocalSetupProviderCfg) -> Self {
        Self {
            name,
            public_env_directory: None,
            provider,
        }
    }
}

impl SetupCfg for LocalSetupCfg {
    fn name(&self) -> String {
        self.name.to_owned()
    }
}

impl EnvPathCfg for LocalSetupCfg {
    fn env_path_op(&self) -> Option<&PathBuf> {
        self.public_env_directory.as_ref()
    }

    fn set_env_path_op(&mut self, directory: Option<PathBuf>) {
        self.public_env_directory = directory
    }
}
