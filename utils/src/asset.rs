//! Embedding and shifting of asset
use super::error::Error;
use std::collections::HashMap;
use std::error::Error as stdError;
use std::fs;
use std::path::{Path, PathBuf};

pub const ASSETS_DIRECTORY: &'static str = "assets";
pub struct SubAssets {
    all: HashMap<&'static str, &'static str>,
    sub_path: PathBuf,
}

pub enum Assets {
    Sub(SubAssets),
    All(HashMap<&'static str, &'static str>),
}

/// Copy all [`Asset`] in target directory [`path`].
/// With third parameter [`sub_path`] you can copy from a sub assets directory.
pub fn to_dir<P: AsRef<Path>>(path: P, assets: Assets) -> Result<(), Box<dyn stdError>> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(Error::new_box(format!("directory {:?} not exists", path)));
    }

    let assets = match assets {
        Assets::Sub(sub_assets) => {
            let sub_path = sub_assets.sub_path;
            sub_assets
                .all
                .into_iter()
                .filter_map(|(asset_path, content)| -> _ {
                    let p = PathBuf::from(asset_path);
                    let p = p.strip_prefix(sub_path.clone()).ok();

                    // TODO: Try to refactor with .map Option
                    if let Some(p) = p {
                        if let Some(p) = p.to_str() {
                            Some((PathBuf::from(p), content))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<HashMap<_, _>>()
        }
        Assets::All(assets) => assets
            .into_iter()
            .map(|(asset_path, content)| -> _ { (PathBuf::from(asset_path), content) })
            .collect(),
    };

    for (asset_path, contents) in assets {
        let asset_path = PathBuf::from(asset_path);
        let path = path.join(asset_path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(&path, &contents)?;
    }

    Ok(())
}

pub fn default_assets(assets: HashMap<&'static str, &'static str>) -> Assets {
    Assets::Sub(SubAssets {
        sub_path: PathBuf::from(ASSETS_DIRECTORY),
        all: assets,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assets::get_all;
    use std::fs::read_dir;
    use tempdir::TempDir;

    #[allow(unreachable_patterns)]
    #[test]
    fn copy_all_assets_to_target_directory() {
        let tempdir = TempDir::new("copy_all_assets_to_target_directory").unwrap();
        let tempdir = tempdir.path();
        to_dir(&tempdir, default_assets(get_all())).unwrap();
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
