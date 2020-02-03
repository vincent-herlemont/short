//! File manipulation operations related of d4d domain.
use super::io;
use crate::lib::error::Error;
use crate::lib::result::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// SuperStructure including [`PathBuf`] of files and his content.
#[derive(Debug)]
pub struct ContentFile {
    path: PathBuf,
    contents: String,
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
    use crate::lib::test::{after, before};

    #[test]
    fn read_contain_multi_test() {
        let tmpath = before("search_test");
        let paths = retrieve(&tmpath).unwrap();
        let (content_files, errors) =
            ContentFile::read_contain_multi(&paths, |line| line.contains("test"));

        // ContentFiles
        assert_eq!((&content_files).len(), 1);
        assert_eq!((&content_files)[0].contents, "console.log(\'test.js\');");
        assert!((&content_files)[0]
            .path
            .to_str()
            .unwrap()
            .contains("test.js"));

        // Errors
        assert_eq!((&errors).len(), 2);
        match &errors[0] {
            Error::Io(_) => assert!(true),
            _ => assert!(false),
        }
        match &errors[1] {
            Error::Other(_) => assert!(true),
            _ => assert!(false),
        }

        after(tmpath);
    }
}
