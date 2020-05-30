use crate::cfg::{LocalSetupCfg, SetupCfg};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalProjectSetupCfg {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    private_env_dir: Option<PathBuf>,
}

impl GlobalProjectSetupCfg {
    pub fn new(name: String) -> Self {
        Self {
            name,
            private_env_dir: None,
        }
    }

    pub fn private_env_dir(&self) -> Option<&PathBuf> {
        self.private_env_dir.as_ref()
    }

    pub fn set_private_env_dir(&mut self, dir: PathBuf) {
        self.private_env_dir = Some(dir)
    }
}

impl From<&LocalSetupCfg> for GlobalProjectSetupCfg {
    fn from(local_setup: &LocalSetupCfg) -> Self {
        Self {
            name: local_setup.name().clone(),
            private_env_dir: None,
        }
    }
}

impl SetupCfg for GlobalProjectSetupCfg {
    fn name(&self) -> &String {
        &self.name
    }

    fn rename(&mut self, name: &String) {
        self.name = name.clone();
    }
}
