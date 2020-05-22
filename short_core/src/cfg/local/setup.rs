use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::cfg::local::EnvGroup;
use crate::cfg::setup::SetupCfg;
use crate::cfg::EnvPathCfg;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalSetupCfg {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    public_env_dir: Option<PathBuf>,

    file: PathBuf,

    env_groups: Rc<RefCell<Vec<EnvGroup>>>,
}

impl LocalSetupCfg {
    pub fn new(name: String, file: PathBuf) -> Self {
        Self {
            name,
            public_env_dir: None,
            file,
            env_groups: Rc::new(RefCell::new(vec![EnvGroup {}])),
        }
    }
}

impl SetupCfg for LocalSetupCfg {
    fn name(&self) -> &String {
        &self.name
    }

    fn rename(&mut self, name: &String) {
        self.name = name.clone();
    }
}

impl EnvPathCfg for LocalSetupCfg {
    fn env_path_op(&self) -> Option<&PathBuf> {
        self.public_env_dir.as_ref()
    }

    fn set_env_path_op(&mut self, dir: Option<PathBuf>) {
        self.public_env_dir = dir
    }
}
