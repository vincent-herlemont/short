//! Helper for test related of d4d domain.

use std::fs;
use std::path::PathBuf;

use tempdir::TempDir;

use crate::resource;

pub struct Config {
    pub tmp_dir: PathBuf,
}

impl Drop for Config {
    fn drop(&mut self) {
        fs::remove_dir_all(self.tmp_dir.clone()).expect("can not clean tmp directory");
    }
}

/// Return [`InspectorConfig`], create temporary directory and copy resource on it.
///
/// The temporary directory is owned by [`InspectorConfig.path`].
///
/// # Recommendation
///
/// Need to call [`after`] at the end of test.
pub fn before(test_name: &str) -> Config {
    let test_name = format!("{}.{}", "d4d", test_name);

    // Create temporary directory.
    let path = TempDir::new(test_name.as_str())
        .expect("fail to create temporary directory")
        .into_path();

    // Copy resources to it.
    resource::to_dir(&path).expect("fail to copy resources");

    Config { tmp_dir: path }
}
