use anyhow::{Context, Result};
use fs_extra::copy_items;
use fs_extra::dir;
use git2::Repository;
use std::fs::read_dir;
use std::path::PathBuf;

pub struct Template {
    name: String,
    url: String,
    dir: Option<PathBuf>,
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
            .context("fail to clone repository")?;
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
    use crate::template::template::Template;
    use cli_integration_test::IntegrationTestEnvironment;

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

        template.checkout(e.path().join(TEMPLATE_TMP_DIR)).unwrap();
        template.copy(e.path().join(TEMPLATE_TARGET_DIR)).unwrap();

        assert!(e.path().join(TEMPLATE_TARGET_DIR).join("run.sh").exists());
        assert!(e.path().join(TEMPLATE_TARGET_DIR).join("env/.dev").exists());
        assert!(!e.path().join(TEMPLATE_TARGET_DIR).join(".git").exists());

        assert!(e.path().join(TEMPLATE_TMP_DIR).join("run.sh").exists());
        assert!(e.path().join(TEMPLATE_TMP_DIR).join("env/.dev").exists());
        assert!(e.path().join(TEMPLATE_TMP_DIR).join(".git").exists());
    }
}
