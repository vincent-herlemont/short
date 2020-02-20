//! Build d4d project
//! Create gen.assets.rs file TODO package on the crate.
use std::path::PathBuf;
use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Error as ioError,
    io::Write,
    path::Path,
};

pub const ASSETS_DIRECTORY: &'static str = "assets";

// TODO: watch only write file https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorerun-if-changedpath
fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir)
        .join("../../../../../d4d/src/")
        .canonicalize()?
        .to_path_buf();

    let asset_paths = retrieve(&out_dir)?;
    let asset_paths = asset_paths
        .into_iter()
        .filter_map(|p| -> _ {
            if !p.is_file() {
                return None;
            }
            let p = p.strip_prefix(out_dir.to_owned()).unwrap().to_path_buf();
            if let Some(s) = p.to_str() {
                if s.starts_with(ASSETS_DIRECTORY) {
                    return Some(p);
                }
            }
            None
        })
        .collect::<Vec<PathBuf>>();

    let dest_path = Path::new(&out_dir).join("gen_assets.rs");

    generate_assets_file(&dest_path, asset_paths)
}

fn retrieve(path: &PathBuf) -> Result<Vec<PathBuf>, ioError> {
    let mut child_entries: Vec<PathBuf> = Vec::new();

    let entries = fs::read_dir(&path)?
        .filter_map(|result| {
            result
                .map(|dir_entry| {
                    let path = dir_entry.path();
                    if path.is_dir() {
                        if let Ok(new_child_entries) = retrieve(&path) {
                            child_entries.append(&mut (new_child_entries.to_vec()))
                        }
                    }
                    path
                })
                .ok()
        })
        .collect::<Vec<PathBuf>>();

    Ok([entries, child_entries].concat())
}

/// Generate assets files
fn generate_assets_file(path: &PathBuf, asset_paths: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut all_the_files = File::create(path)?;

    writeln!(&mut all_the_files, r#"use utils::asset::Asset;"#,)?;
    writeln!(&mut all_the_files, r#""#,)?;
    writeln!(&mut all_the_files, r#"/// Get all [`Asset`]"#,)?;
    writeln!(&mut all_the_files, r#"#[allow(dead_code)]"#,)?;
    writeln!(
        &mut all_the_files,
        r#"pub fn get_assets() -> Vec<Asset> {{"#,
    )?;
    writeln!(&mut all_the_files, r#"    vec!["#,)?;

    for f in asset_paths {
        writeln!(&mut all_the_files, r#"        Asset::new("#,)?;
        writeln!(
            &mut all_the_files,
            r#"            "{name}","#,
            name = f.as_path().display()
        )?;
        writeln!(
            &mut all_the_files,
            r#"            include_str!("{name}"),"#,
            name = f.as_path().display()
        )?;
        writeln!(&mut all_the_files, r#"        ),"#,)?;
    }

    writeln!(&mut all_the_files, r#"    ]"#,)?;
    writeln!(&mut all_the_files, r#"}}"#,)?;

    Ok(())
}
