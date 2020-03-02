//! Path manipulation operations related of d4d domain.
use std::fs;
use std::io;
use std::path::PathBuf;

/// Retrieve recursively all path available at the current [`path`]
///
/// ### Error
///
/// Return only [`io::Error`] related to the reading of the root [`path`] directory.
pub fn retrieve(path: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
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

/// Return [`Vec<PathBuf>`] who contain the [`extensions`].
pub fn filter_extensions(paths: &[PathBuf], extensions: &[&str]) -> Vec<PathBuf> {
    paths
        .iter()
        .filter(|path| {
            if let Some(extension) = path.extension() {
                if let Some(extension) = extension.to_str() {
                    return extensions.contains(&extension);
                }
            }
            false
        })
        .cloned()
        .collect::<Vec<PathBuf>>()
}

#[cfg(test)]
mod tests {
    use crate::assets::get_all;
    use crate::path::{filter_extensions, retrieve};
    use crate::test::before;
    use std::path::PathBuf;

    #[allow(unreachable_patterns)]
    #[test]
    fn retrieve_entries_test() {
        let config = before("before_test", &get_all());
        let mut entries = retrieve(&config.tmp_dir).unwrap();
        entries.sort();
        assert!(entries.len() >= 3);
        assert_find!(
            entries,
            entry,
            entry.strip_prefix(&config.tmp_dir).unwrap()
                == &PathBuf::from("valid_aws_template.yaml")
        );
    }

    #[test]
    fn filter_extensions_test() {
        let paths = [
            PathBuf::from("/test/test"),
            PathBuf::from("/test/test.yaml"),
            PathBuf::from("/test/test.txt"),
            PathBuf::from("/test/test.js"),
        ];

        let filtered_extensions = filter_extensions(&paths, &["js", "html"]);
        assert_eq!(&filtered_extensions, &vec![PathBuf::from("/test/test.js")]);
        assert_eq!((&filtered_extensions.clone()).len(), 1 as usize);
    }
}
