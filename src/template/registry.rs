// TODO : In the future registry will be downloaded from this repository.
// https://github.com/vincent-herlemont/short-template-index

use crate::template::template::Template;
use anyhow::{Context, Result};
use std::collections::HashMap;

type Name = String;
type Url = String;

pub struct Registry {}

impl Registry {
    pub fn new() -> Self {
        Self {}
    }

    pub fn index() -> HashMap<Name, Url> {
        let mut data = HashMap::new();
        data.insert(
            "aws-sam".to_string(),
            "https://github.com/vincent-herlemont/aws-sam-short-template.git".to_string(),
        );
        data.insert(
            "test".to_string(),
            "https://github.com/vincent-herlemont/test-short-template.git".to_string(),
        );
        data
    }

    pub fn get(&self, name: &str) -> Result<Template> {
        let index = Registry::index();
        let url = index
            .get(name)
            .context(format!("template `{}` not found", name))?;
        let template = Template::new(name.to_string(), url.to_string());
        Ok(template)
    }
}
