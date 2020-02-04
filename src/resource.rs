//! Embedding and shifting of resources
use std::error::Error as stdError;
use std::fs;
use std::path::{Path, PathBuf};

use crate::lib::error::Error;

#[derive(Debug)]
pub struct Resource {
    path: PathBuf,
    pub data: String,
}

impl Resource {
    fn new(path: &str, data: &str) -> Resource {
        Resource {
            path: PathBuf::from(path),
            data: data.to_string(),
        }
    }

    fn path(&self) -> &Path {
        return &self.path.as_path();
    }
}

/// Copy all [`Resource`] in target directory [`path`].
pub fn to_dir(path: &Path) -> Result<(), Box<dyn stdError>> {
    if !path.exists() {
        return Err(Error::new_box(format!("directory {:?} not exists", path)));
    }
    for resource in get() {
        let resource_path = resource.path();
        let path = path.join(resource_path.strip_prefix(RESOURCE_DIRECTORY)?);
        let contents = resource.data.as_str();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(&path, &contents)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_dir;
    use tempdir::TempDir;

    #[test]
    fn get_all_resources() {
        let resources = get();
        assert!(resources.first().unwrap().path.to_str().unwrap().len() > 0);
        assert!(resources.first().unwrap().data.len() > 0);
    }

    #[test]
    fn copy_all_resources_to_target_directory() {
        let tempdir = TempDir::new("copy_all_resources_to_target_directory").unwrap();
        let tempdir = tempdir.path();
        to_dir(&tempdir).unwrap();
        let mut files: Vec<_> = read_dir(&tempdir)
            .unwrap()
            .map(|o| o.unwrap().path())
            .collect();
        files.sort();
        assert_eq!(
            &files[0].strip_prefix(&tempdir).unwrap(),
            &Path::new("certificate.yaml")
        );
        assert_eq!(
            &files[1].strip_prefix(&tempdir).unwrap(),
            &Path::new("test")
        );
    }
}

const RESOURCE_DIRECTORY: &'static str = "./init_tpl";

/// Get all [`Resource`]
pub fn get() -> Vec<Resource> {
    vec![
        Resource::new(
            "./init_tpl/certificate.yaml",
            include_str!("./init_tpl/certificate.yaml"),
        ),
        Resource::new(
            "./init_tpl/test/test.js",
            include_str!("./init_tpl/test/test.js"),
        ),
    ]
}
