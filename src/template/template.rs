use std::fs::read_dir;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use anyhow::{Context, Result};
use fs_extra::copy_items;
use fs_extra::dir;
use git2::Repository;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct Template {
    #[serde(skip)]
    name: String,
    url: String,
    #[serde(skip)]
    dir: Option<PathBuf>,
}

impl Hash for Template {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Template {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Template {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn url(&self) -> &String {
        &self.url
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Template {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            dir: None,
        }
    }

    pub fn checkout(&mut self, target_dir: PathBuf) -> Result<()> {
        Repository::clone(self.url.as_str(), target_dir.as_path())
            .context("fail to clone templates repository")?;
        self.dir = Some(target_dir);
        Ok(())
    }

    pub fn copy(&self, target_dir: PathBuf) -> Result<()> {
        let dir = self.dir.as_ref().context("please checkout before copy")?;
        let paths: Vec<_> = read_dir(dir)?
            .into_iter()
            .filter_map(|e| {
                if let Ok(e) = e {
                    let path = e.path();
                    if let Ok(striped_path) = path.strip_prefix(dir) {
                        let str = striped_path.to_string_lossy();
                        if !str.starts_with(".git")
                            && !str.is_empty()
                            && str.into_owned() != "short.yaml"
                        {
                            return Some(path.to_path_buf());
                        }
                    }
                }
                None
            })
            .collect();

        let options = dir::CopyOptions::new();
        copy_items(&paths, target_dir, &options)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use cli_integration_test::IntegrationTestEnvironment;

    use crate::template::template::Template;

    const TEMPLATE_TMP_DIR: &'static str = "tmp";
    const TEMPLATE_TARGET_DIR: &'static str = "target";

    #[test]
    fn checkout() {
        let mut e = IntegrationTestEnvironment::new("checkout");
        e.add_dir(TEMPLATE_TMP_DIR);
        e.add_dir(TEMPLATE_TARGET_DIR);
        e.setup();

        let mut template = Template::new(
            "".to_string(),
            "https://github.com/vincent-herlemont/test-short-template.git".to_string(),
        );

        template
            .checkout(e.path().unwrap().join(TEMPLATE_TMP_DIR))
            .unwrap();
        template
            .copy(e.path().unwrap().join(TEMPLATE_TARGET_DIR))
            .unwrap();

        assert!(e
            .path()
            .unwrap()
            .join(TEMPLATE_TARGET_DIR)
            .join("run.sh")
            .exists());
        assert!(e
            .path()
            .unwrap()
            .join(TEMPLATE_TARGET_DIR)
            .join("env/.dev")
            .exists());
        assert!(!e
            .path()
            .unwrap()
            .join(TEMPLATE_TARGET_DIR)
            .join(".git")
            .exists());

        assert!(e
            .path()
            .unwrap()
            .join(TEMPLATE_TMP_DIR)
            .join("run.sh")
            .exists());
        assert!(e
            .path()
            .unwrap()
            .join(TEMPLATE_TMP_DIR)
            .join("env/.dev")
            .exists());
        assert!(e
            .path()
            .unwrap()
            .join(TEMPLATE_TMP_DIR)
            .join(".git")
            .exists());
    }
}
