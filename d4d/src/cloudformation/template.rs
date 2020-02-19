//! Cloudformation template description
use serde::Deserialize;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Template {
    pub nested: Vec<Rc<RefCell<Template>>>,
    pub content_template: ContentTemplate,
}

/// Template aws file
#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct ContentTemplate {
    #[serde(rename(deserialize = "Description"))]
    pub description: Option<String>,

    #[serde(rename(deserialize = "AWSTemplateFormatVersion"))]
    pub aws_template_format_version: String,
}
