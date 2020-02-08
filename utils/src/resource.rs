//! Embedding and shifting of resources
//! TODO: add and create an abstraction on utils crate.
use super::error::Error;
use crate::test::{get_resource, TEST_RESOURCE_DIRECTORY};
use std::error::Error as stdError;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Resource {
    path: PathBuf,
    pub data: String,
}

impl Resource {
    pub fn new(path: &str, data: &str) -> Resource {
        Resource {
            path: PathBuf::from(path),
            data: data.to_string(),
        }
    }

    pub fn path(&self) -> &Path {
        return &self.path.as_path();
    }
}

/// Copy all [`Resource`] in target directory [`path`].
pub fn to_dir(path: &Path) -> Result<(), Box<dyn stdError>> {
    if !path.exists() {
        return Err(Error::new_box(format!("directory {:?} not exists", path)));
    }
    for resource in get_resource() {
        let resource_path = resource.path();
        let path = path.join(resource_path.strip_prefix(TEST_RESOURCE_DIRECTORY)?);
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
        let resources = get_resource();
        assert!(resources.first().unwrap().path.to_str().unwrap().len() > 0);
        assert!(resources.first().unwrap().data.len() > 0);
    }

    #[allow(unreachable_patterns)]
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
        assert_find!(
            files,
            dir_entry,
            dir_entry.strip_prefix(&tempdir).unwrap() == Path::new("1_certificate.yaml")
        );
        assert_find!(
            files,
            dir_entry,
            dir_entry.strip_prefix(&tempdir).unwrap() == Path::new("1_certificate_altered.yaml")
        );
        assert_find!(
            files,
            dir_entry,
            dir_entry.strip_prefix(&tempdir).unwrap() == Path::new("3_test")
        );
    }
}
