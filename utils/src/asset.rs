//! Embedding and shifting of asset
use super::error::Error;
use crate::test::TEST_ASSETS_DIRECTORY;
use std::error::Error as stdError;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Asset {
    path: PathBuf,
    pub data: String,
}

impl Asset {
    pub fn new(path: &str, data: &str) -> Asset {
        Asset {
            path: PathBuf::from(path),
            data: data.to_string(),
        }
    }

    pub fn path(&self) -> &Path {
        return &self.path.as_path();
    }
}

/// Copy all [`Asset`] in target directory [`path`].
pub fn to_dir(path: &Path, assets: &[Asset]) -> Result<(), Box<dyn stdError>> {
    if !path.exists() {
        return Err(Error::new_box(format!("directory {:?} not exists", path)));
    }
    for asset in assets {
        let asset_path = asset.path();
        let path = path.join(asset_path.strip_prefix(TEST_ASSETS_DIRECTORY)?);
        let contents = asset.data.as_str();
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
    use crate::test::get_assets;
    use std::fs::read_dir;
    use tempdir::TempDir;

    #[test]
    fn get_assets_test() {
        let assets = get_assets();
        assert!(assets.first().unwrap().path.to_str().unwrap().len() > 0);
        assert!(assets.first().unwrap().data.len() > 0);
    }

    #[allow(unreachable_patterns)]
    #[test]
    fn copy_all_assets_to_target_directory() {
        let tempdir = TempDir::new("copy_all_assets_to_target_directory").unwrap();
        let tempdir = tempdir.path();
        to_dir(&tempdir, &get_assets()).unwrap();
        let files: Vec<_> = read_dir(&tempdir)
            .unwrap()
            .map(|o| o.unwrap().path())
            .collect();
        assert_find!(
            files,
            dir_entry,
            dir_entry.strip_prefix(&tempdir).unwrap() == Path::new("valid_aws_template.yaml")
        );
        assert_find!(
            files,
            dir_entry,
            dir_entry.strip_prefix(&tempdir).unwrap() == Path::new("altered_aws_template.yaml")
        );
        assert_find!(
            files,
            dir_entry,
            dir_entry.strip_prefix(&tempdir).unwrap() == Path::new("test")
        );
    }
}
