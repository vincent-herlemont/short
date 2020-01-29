//! Inspection and manipulation of cloudformation templates.

use std::path::PathBuf;
use std::fs;
use std::io;
use serde::export::Vec;

/// The configuration of the cloudformation inspector.
#[derive(Debug)]
struct InspectorConfig {
    path:PathBuf,
}

/// Retrieve recursively all entries available at the current [`path`]
///
/// ### Error
///
/// Return only [`io::Error`] related to the reading of the root [`path`] directory.
fn retrieve_entries(path: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {

    let mut child_entries:Vec<PathBuf> = Vec::new();

    let entries = fs::read_dir(&path)?
        .filter_map(|el|
            el.map(|el| {
                let path = el.path();
                if path.is_dir() {
                    if let Ok(new_child_entries) = retrieve_entries(&path) {
                        child_entries.append(&mut (new_child_entries.to_vec()))
                    }
                }
                path
            }).ok()
        ).collect::<Vec<_>>();

    Ok([entries,child_entries].concat())
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use crate::cloudformation::{InspectorConfig, retrieve_entries};
    use crate::resource;
    use std::path::PathBuf;

    /// Return [`InspectorConfig`], create temporary directory and copy resource on it.
    ///
    /// The temporary directory is owned by [`InspectorConfig.path`].
    /// And will be deleted when [`InspectorConfig.path`] will be dropped.
    ///
    fn before(test_name: &str) -> InspectorConfig {

        // Create temporary directory.
        let path = TempDir::new(test_name).unwrap().into_path();

        // Copy resources to it.
        resource::to_dir(&path).unwrap();

        InspectorConfig {
            path
        }
    }

    #[test]
    fn retrieve_entries_test() {
        let icfg = before("before_test");
        let mut entries = retrieve_entries(&icfg.path).unwrap();
        entries.sort();
        assert!(entries.len() >= 3);
        assert_eq!(&entries[0].strip_prefix(icfg.path).unwrap(),&PathBuf::from("certificate.yaml"));
    }
}