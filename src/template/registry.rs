// TODO : In the future registry will be downloaded from this repository.
// https://github.com/vincent-herlemont/short-template-index

use std::collections::HashSet;
use std::fs::{read_dir, read_to_string};
use std::path::Path;

use anyhow::{Context, Result};
use git2::Repository;

use crate::template::template::Template;

const DEFAULT_REPOSITORY_URL: &'static str =
    "https://github.com/vincent-herlemont/short-template-index";

#[derive(Debug)]
pub struct Registry {
    index: HashSet<Template>,
}

impl Registry {
    pub fn checkout(target_dir: &Path) -> Result<Self> {
        let url = option_env!("SHORT_REPOSITORY_URL").unwrap_or(DEFAULT_REPOSITORY_URL);
        Repository::clone(url, target_dir).context("fail to clone registry repository")?;
        let mut registry = Registry {
            index: HashSet::new(),
        };
        for dir_entry in read_dir(target_dir)? {
            let dir_entry = dir_entry?;
            let path = dir_entry.path();
            if path.is_file() {
                let content = read_to_string(&path)?;
                let mut template: Template = serde_json::from_str(&content)?;
                let name = path
                    .file_stem()
                    .context("fail to get file name")?
                    .to_string_lossy()
                    .into_owned();
                template.set_name(name);
                registry.index.insert(template);
            }
        }
        Ok(registry)
    }

    pub fn index(&self) -> &HashSet<Template> {
        &self.index
    }

    pub fn get(&self, name: &str) -> Result<Template> {
        let template = self
            .index
            .iter()
            .find(|template| template.name() == name)
            .context(format!("template `{}` not found", name))?;
        let template = Template::new(template.name().clone(), template.url().clone());
        Ok(template)
    }
}

#[cfg(test)]
mod tests {
    use cli_integration_test::IntegrationTestEnvironment;
    use serde_json::from_str;

    use crate::template::Registry;
    use crate::template::Template;

    #[test]
    fn deserialize_json() {
        let entry: Template = from_str(
            "{ \"url\" : \"https://github.com/vincent-herlemont/test-short-template.git\" }",
        )
        .unwrap();
        assert_eq!(
            entry.url(),
            "https://github.com/vincent-herlemont/test-short-template.git"
        );
    }

    #[test]
    fn checkout() {
        let e = IntegrationTestEnvironment::new("registry_checkout");
        let registry = Registry::checkout(e.path().unwrap().as_ref()).unwrap();

        let _template = registry.get("test").unwrap();
    }
}
