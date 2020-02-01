//! Helper for test related of d4d domain.

use std::fs;
use std::path::PathBuf;

use tempdir::TempDir;

use crate::resource;

/// Return [`InspectorConfig`], create temporary directory and copy resource on it.
///
/// The temporary directory is owned by [`InspectorConfig.path`].
/// And will be deleted when [`InspectorConfig.path`] will be dropped.
///
/// # Recommendation
///
/// Need to call [`after`] at the end of test.
pub fn before(test_name: &str) -> PathBuf {
    let test_name = format!("{}.{}", "d4d", test_name);

    // Create temporary directory.
    let path = TempDir::new(test_name.as_str())
        .expect("fail to create temporary directory")
        .into_path();

    // Copy resources to it.
    resource::to_dir(&path).expect("fail to copy resources");

    path
}

/// Clean test
///
/// # Recommendation
///
/// Need to be call at the end of the test who use [`before`].
pub fn after(path: PathBuf) {
    // Clean tmp directory
    fs::remove_dir_all(path).expect("can not clean tmp directory");
}
