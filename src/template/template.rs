use anyhow::Result;
use git2::Repository;
use std::fs::read_dir;
use std::path::PathBuf;

pub struct Template {
    url: String,
    tmp_dir: PathBuf,
}
use fs_extra::copy_items;
use fs_extra::dir;


impl Template {
    pub fn new(url: String, tmp_dir: PathBuf) -> Self {
        Self { url, tmp_dir }
    }

    pub fn checkout(&self, target_dir: PathBuf) -> Result<()> {
        let tmp_dir = self.tmp_dir.as_path();
        Repository::clone(self.url.as_str(), tmp_dir)?;

        let paths: Vec<_> = read_dir(tmp_dir)?
            .into_iter()
            .filter_map(|e| {
                if let Ok(e) = e {
                    let path = e.path();
                    if let Ok(striped_path) = path.strip_prefix(tmp_dir) {
                        let str = striped_path.to_string_lossy();
                        if !str.starts_with(".git") && !str.is_empty() {
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

        let template = Template::new(
            "https://github.com/vincent-herlemont/test-short-template.git".to_string(),
            e.path().join(TEMPLATE_TMP_DIR),
        );

        template
            .checkout(e.path().join(TEMPLATE_TARGET_DIR))
            .unwrap();

        assert!(e.path().join(TEMPLATE_TARGET_DIR).join("run.sh").exists());
        assert!(e.path().join(TEMPLATE_TARGET_DIR).join("env/.dev").exists());
        assert!(!e.path().join(TEMPLATE_TARGET_DIR).join(".git").exists());

        assert!(e.path().join(TEMPLATE_TMP_DIR).join("run.sh").exists());
        assert!(e.path().join(TEMPLATE_TMP_DIR).join("env/.dev").exists());
        assert!(e.path().join(TEMPLATE_TMP_DIR).join(".git").exists());
    }
}
