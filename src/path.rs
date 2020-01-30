//! Path manipulation operations

use std::path::PathBuf;
use std::fs;
use std::io;
use serde::export::Vec;

/// Retrieve recursively all path available at the current [`path`]
///
/// ### Error
///
/// Return only [`io::Error`] related to the reading of the root [`path`] directory.
pub fn retrieve(path: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {

    let mut child_entries:Vec<PathBuf> = Vec::new();

    let entries = fs::read_dir(&path)?
        .filter_map(|result|
            result.map(|dir_entry| {
                let path = dir_entry.path();
                if path.is_dir() {
                    if let Ok(new_child_entries) = retrieve(&path) {
                        child_entries.append(&mut (new_child_entries.to_vec()))
                    }
                }
                path
            }).ok()
        ).collect::<Vec<PathBuf>>();

    Ok([entries,child_entries].concat())
}

/// Return [`Vec<PathBuf>`] who contain the [`extensions`].
pub fn filter_extensions(paths:&[PathBuf],extensions:&[&str]) -> Vec<PathBuf> {
    paths.iter().filter(|path| {
        if let Some(extension) = path.extension() {
            if let Some(extension) = extension.to_str() {
                return extensions.contains(&extension)
            }
        }
        false
    }).cloned().collect::<Vec<PathBuf>>()
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use crate::resource;
    use std::path::PathBuf;
    use crate::path::{retrieve, filter_extensions};

    /// Return [`InspectorConfig`], create temporary directory and copy resource on it.
    ///
    /// The temporary directory is owned by [`InspectorConfig.path`].
    /// And will be deleted when [`InspectorConfig.path`] will be dropped.
    ///
    fn before(test_name: &str) -> PathBuf {
        // Create temporary directory.
        let path = TempDir::new(test_name).unwrap().into_path();
        // Copy resources to it.
        resource::to_dir(&path).unwrap();
        path
    }

    #[test]
    fn retrieve_entries_test() {
        let path = before("before_test");
        let mut entries = retrieve(&path).unwrap();
        entries.sort();
        assert!(entries.len() >= 3);
        assert_eq!(&entries[0].strip_prefix(&path).unwrap(),&PathBuf::from("certificate.yaml"));
    }

    #[test]
    fn filter_extensions_test() {
        let paths = [
            PathBuf::from("/test/test"),
            PathBuf::from("/test/test.yaml"),
            PathBuf::from("/test/test.txt"),
            PathBuf::from("/test/test.js"),
        ];

        let filtered_extensions = filter_extensions(&paths,&["js","html"]);
        assert_eq!(&filtered_extensions, &vec![PathBuf::from("/test/test.js")]);
        assert_eq!((&filtered_extensions.clone()).len(), 1 as usize);
    }
}