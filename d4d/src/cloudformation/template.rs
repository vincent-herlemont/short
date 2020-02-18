//! Cloudformation template description
use serde::Deserialize;
use std::cell::RefCell;
use std::rc::Rc;

/// Template aws file
#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Template {
    #[serde(rename(deserialize = "Description"))]
    pub description: Option<String>,

    #[serde(rename(deserialize = "AWSTemplateFormatVersion"))]
    pub aws_template_format_version: String,

    /// Correspond to nested stacks https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html
    #[serde(skip)]
    pub nested: Vec<Rc<RefCell<Template>>>,
}
