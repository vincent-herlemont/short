//! File manipulation operations related of d4d domain.
use super::io;
use crate::lib::error::Error;
use crate::lib::result::Result;
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
        let (content_files, errors): (Vec<_>, Vec<_>) = paths
            .into_iter()
            .map(|path| ContentFile::read_contain(path, &next_line))
            .partition(Result::is_ok);
        (
            content_files.into_iter().map(Result::unwrap).collect(),
            errors.into_iter().map(Result::unwrap_err).collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::error::Error;
    use crate::lib::fs::ContentFile;
    use crate::lib::path::retrieve;
    use crate::lib::test::before;

    #[test]
    fn read_contain_multi_test() {
        let config = before("search_test");
        let paths = retrieve(&config.tmp_dir).unwrap();
        let (mut content_files, errors) =
            ContentFile::read_contain_multi(&paths, |line| line.contains("test"));

        content_files.sort();
        // ContentFiles
        assert_eq!((&content_files).len(), 2);
        assert_eq!((&content_files)[0].contents, "console.log(\'test.js\');");
        assert!((&content_files)[0]
            .path
            .to_str()
            .unwrap()
            .contains("test.js"));

        // Errors
        assert_eq!((&errors).len(), 3);
        match &errors[0] {
            Error::Io(_) => assert!(true),
            _ => assert!(false),
        }
        match &errors[1] {
            Error::Other(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
