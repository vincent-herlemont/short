//! Embedding and shifting of asset
use crate::error::Error;
use crate::result::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const ASSETS_DIRECTORY: &'static str = "assets";
/// Sub Assets allow to insert from a sub set directory
/// into the current directory without create [`sub_path`] directories.
pub struct SubAssets {
    all: HashMap<&'static str, &'static str>,
    sub_path: PathBuf,
}

pub enum Assets {
    StaticSub(SubAssets),
    Static(HashMap<&'static str, &'static str>),
    Dynamic(HashMap<PathBuf, String>),
    None,
}

/// Copy all [`Asset`] in target directory [`path`].
/// With third parameter [`sub_path`] you can copy from a sub assets directory.
pub fn to_dir<P: AsRef<Path>>(path: P, assets: Assets) -> Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(Error::from(format!("directory {:?} not exists", path)));
    }

    let assets = match assets {
        Assets::StaticSub(sub_assets) => {
            let sub_path = sub_assets.sub_path;
            sub_assets
                .all
                .into_iter()
                .filter_map(|(asset_path, content)| -> _ {
                    let p = PathBuf::from(asset_path);
                    let p = p.strip_prefix(sub_path.clone()).ok();

                    if let Some(p) = p {
                        p.to_str()
                            .map(|s| -> _ { (PathBuf::from(s), String::from(content)) })
                    } else {
                        None
                    }
                })
                .collect::<HashMap<_, _>>()
        }
        Assets::Static(assets) => assets
            .into_iter()
            .map(|(asset_path, content)| -> _ {
                (PathBuf::from(asset_path), String::from(content))
            })
            .collect(),
        Assets::Dynamic(assets) => assets,
        Assets::None => HashMap::new(),
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
    Assets::StaticSub(SubAssets {
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
