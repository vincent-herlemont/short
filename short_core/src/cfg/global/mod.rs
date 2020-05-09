mod project;
mod setup;

use serde::{Deserialize, Serialize};

use crate::cfg::global::project::GlobalProjectCfg;
use crate::cfg::new::NewCfg;

pub const GLOCAL_FILE_NAME: &'static str = "cfg.yml";

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalCfg {
    projects: Vec<GlobalProjectCfg>,
}

impl NewCfg for GlobalCfg {
    type T = Self;
    fn new() -> Self {
        Self { projects: vec![] }
    }
}
