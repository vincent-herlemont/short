//! Embedding and shifting of resources
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use super::d4d_error;

#[derive(Debug)]
pub struct Resource {
    pub path: String, // TODO : change to `PathBuf`
    pub data: String,
}

impl Resource {
    fn new(path: &str, data: &str) -> Resource {
        Resource {
            path: path.to_string(),
            data: data.to_string(),
        }
    }
}

/// Copy all [`Resource`] in target directory [`path`].
pub fn to_dir(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        return Err(Box::new(d4d_error::new("can not find directory"))); // TODO: make error abstraction
    }
    for resource in get() {
        let resource_path = Path::new(resource.path.as_str());
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
    use std::fs::read_dir;

    use tempdir::TempDir;

    use super::*;

    #[test]
    fn get_all_resources() {
        let resources = get();
        assert!(resources.first().unwrap().path.len() > 0);
        assert!(resources.first().unwrap().data.len() > 0);
    }

    #[test]
    fn copy_all_resources_to_target_directory() {
        let tempdir = TempDir::new("copy_all_resources_to_target_directory").unwrap();
        let tempdir = tempdir.into_path();
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
