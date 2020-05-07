use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum LocalSetupProviderCfg {
    #[serde(rename = "cloudformation")]
    Cloudformation(LocalSetupProviderCloudformationCfg),

    #[serde(rename = "unknown")]
    None,
}

impl LocalSetupProviderCfg {
    pub fn new_none() -> Self {
        LocalSetupProviderCfg::None
    }
    pub fn new_cloudformation(template: PathBuf) -> Self {
        LocalSetupProviderCfg::Cloudformation(LocalSetupProviderCloudformationCfg { template })
    }
}

#[derive(Debug, Serialize, Deserialize)]

pub struct LocalSetupProviderCloudformationCfg {
    template: PathBuf,
}
