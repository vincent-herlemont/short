//! File manipulation operations related of short domain.
use super::io;
use crate::error::Error;
use crate::result::{unwrap_partition, Result};
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// SuperStructure including [`PathBuf`] of files and his content.
#[derive(Debug, Eq, PartialEq)]
pub struct ContentFile {
    pub path: PathBuf,
    pub contents: String,
}

impl Ord for ContentFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialOrd for ContentFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

impl ContentFile {
    /// Return [`ContentFile`] when the predicate [`next_line`] is satisfied.
    pub fn read_contain<F>(path: &PathBuf, next_line: F) -> Result<ContentFile>
    where
        F: Fn(&str) -> bool,
    {
        let contents = match File::open(path).map(|file| BufReader::new(file)) {
            Ok(buffer) => io::read_to_string_contain(buffer, next_line),
            Err(error) => Err(Error::from(error)),
        };

        match contents {
            Ok(contents) => Ok(ContentFile {
                path: path.to_owned(),
                contents,
            }),
            Err(error) => Err(error),
        }
    }

    /// Return each [`ContentFile`] and [`Error`] when the predicate [`next_line`] is satisfied or not.
    pub fn read_contain_multi<F>(paths: &[PathBuf], next_line: F) -> (Vec<ContentFile>, Vec<Error>)
    where
        F: Fn(&str) -> bool,
    {
        let results: (Vec<_>, Vec<_>) = paths
            .into_iter()
            .map(|path| ContentFile::read_contain(path, &next_line))
            .partition(Result::is_ok);
        unwrap_partition(results)
    }
}

#[cfg(test)]
mod tests {
    use crate::asset::default_assets;
    use crate::assets::get_all;
    use crate::error::Error;
    use crate::fs::ContentFile;
    use crate::path::retrieve;
    use crate::test::before;
    use std::io;

    #[allow(unreachable_patterns)]
    #[test]
    fn read_contain_multi_test() {
        let config = before("search_test", default_assets(get_all()));
        let paths = retrieve(&config.tmp_dir).unwrap();
        let (mut content_files, errors) =
            ContentFile::read_contain_multi(&paths, |line| line.contains("test"));

        content_files.sort();
        // ContentFiles
        assert!((&content_files).len() >= 2);
        assert_find!(
            content_files,
            ContentFile { contents, path },
            contents == &String::from("console.log(\'test.js\');")
                && path.to_string_lossy().contains("test.js")
        );

        assert!((&errors).len() > 3);
        assert_find!(errors, Error::Io(e), e.kind() == io::ErrorKind::Other);
        assert_find!(errors, Error::Io(e), e.kind() == io::ErrorKind::NotFound);
    }
}
