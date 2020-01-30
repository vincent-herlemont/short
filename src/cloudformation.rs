//! Inspection and manipulation of cloudformation templates.
use std::path::PathBuf;

/// The configuration of the cloudformation inspector.
#[derive(Debug)]
struct InspectorConfig {
    path:PathBuf,
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use crate::cloudformation::{InspectorConfig};
    use crate::resource;

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
    fn todo_test() {
        let _ = before("toto_test");
    }
}