use serde::{Deserialize, Serialize};

use crate::cfg::new::NewCfg;

pub const GLOCAL_FILE_NAME: &'static str = "cfg.yaml";

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalCfg {}

impl NewCfg for GlobalCfg {
    type T = Self;
    fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalProjectsCfg {}
