//! Cloudformation template description

use serde::Deserialize;

/// Template aws file
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Template {
    #[serde(rename(deserialize = "AWSTemplateFormatVersion"))]
    pub aws_template_format_version: String,
}
