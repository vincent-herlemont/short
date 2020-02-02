//! File manipulation operations related of d4d domain.
use super::io;
use std::fs::File;
use std::io as sio;
use std::io::BufReader;
use std::path::PathBuf;

/// SuperStructure including [`File`] and his content.
#[derive(Debug)]
struct ContentFile {
    path: PathBuf,
    contents: String,
}

fn read_to_string_finds<F>(paths: &[PathBuf], f: F) -> (Vec<ContentFile>, Vec<sio::Error>)
where
    F: Fn(&str) -> bool,
{
    let (content_files, errors): (Vec<_>, Vec<_>) = paths
        .into_iter()
        .map(|path| {
            let contents = match File::open(path).map(|file| BufReader::new(file)) {
                Ok(buffer) => io::read_to_string_finds(buffer, &f),
                Err(error) => Err(error),
            };

            match contents {
                Ok(contents) => Ok(ContentFile {
                    path: path.to_owned(),
                    contents,
                }),
                Err(error) => Err(error),
            }
        })
        .partition(Result::is_ok);

    (
        content_files.into_iter().map(Result::unwrap).collect(),
        errors.into_iter().map(Result::unwrap_err).collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::lib::fs::read_to_string_finds;
    use crate::lib::path::retrieve;
    use crate::lib::test::{after, before};

    #[test]
    fn read_to_string_finds_test() {
        let tmpath = before("search_test");
        let paths = retrieve(&tmpath).unwrap();
        let (content_files, errors) = read_to_string_finds(&paths, |line| line.contains("test"));

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
        after(tmpath);
    }
}
