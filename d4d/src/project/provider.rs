use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};


use utils::error::Error;
use utils::result::Result;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum ProviderCfg {
    #[serde(rename = "aws")]
    ConfAws(AwsCfg),

    #[serde(rename = "unknown")]
    None,
}

impl ProviderCfg {
    pub fn aws(&self) -> Result<&AwsCfg> {
        if let ProviderCfg::ConfAws(conf_aws) = self {
            Ok(conf_aws)
        } else {
            Err(Error::from("aws provider is not defined"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AwsCfg {
    region: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    template_path: Option<PathBuf>,
}

impl AwsCfg {
    pub fn new<N: AsRef<str>>(region: N) -> Self {
        Self {
            region: region.as_ref().to_string(),
            template_path: None,
        }
    }

    pub fn set_template_path<P: AsRef<Path>>(&mut self, template_path: P) -> &Self {
        self.template_path = Some(template_path.as_ref().to_path_buf());
        self
    }

    pub fn template_path(&self) -> Result<PathBuf> {
        self.template_path.clone().ok_or(Error::from(format!(
            "template_path missing", // TODO: set template name
        )))
    }
}
