//! Inspection and manipulation of cloudformation templates.
use crate::lib::error::Error;
use crate::lib::fs::ContentFile;
use crate::lib::path;
use serde::export::fmt::Debug;
use serde::Deserialize;
use std::cmp::Ordering;
use std::path::PathBuf;

static TEMPLATE_VERSION: &str = "2010-09-09";
static YAML_EXTENSIONS: [&str; 2] = ["yaml", "yml"];

/// The configuration of the cloudformation inspector.
#[derive(Debug)]
struct InspectorConfig {
    path: PathBuf,
}

/// File system information
/// TODO: move to another module
#[derive(Debug, PartialEq, Eq)]
struct File {
    content: ContentFile,
    template: Template,
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        self.content.cmp(&other.content)
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.content.cmp(&other.content))
    }
}

/// Template aws file
#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Template {
    #[serde(rename(deserialize = "AWSTemplateFormatVersion"))]
    aws_template_format_version: String,
}

/// Return a list of items [`Result<File>`] matching with AWS cloudformation [`Template`]
///
/// # Errors
///
/// Throw to error [`Error::Other`] error kind TODO: implement domain specific error
fn from_paths(paths: &[PathBuf]) -> (Vec<File>, Vec<Error>) {
    let paths = path::filter_extensions(&paths, &YAML_EXTENSIONS);
    let (content_files, mut errors) =
        ContentFile::read_contain_multi(&paths, |line| line.contains(TEMPLATE_VERSION));

    let (files, errors_files): (Vec<_>, Vec<_>) = content_files
        .into_iter()
        .map(
            |content_file| match serde_yaml::from_str::<Template>(&content_file.contents) {
                Ok(template) => Ok(File {
                    content: content_file,
                    template: template,
                }),
                Err(_) => Err(Error::new(format!(
                    // TODO : Embed serde_yaml::Error to lib::Error.
                    "fail to parse file {}",
                    content_file.path.to_string_lossy()
                ))),
            },
        )
        .partition(Result::is_ok);

    // TODO : Try to abstract this vectored, unwrap and unrap_err.
    let files = files
        .into_iter()
        .map(|file| file.unwrap())
        .collect::<Vec<_>>();

    let mut errors_files = errors_files
        .into_iter()
        .map(|error| error.unwrap_err())
        .collect::<Vec<_>>();

    errors.append(&mut errors_files);

    (files, errors)
}

#[cfg(test)]
mod tests {
    use crate::cloudformation::{from_paths, Template};
    use crate::lib;
    use crate::lib::test::before;

    #[test]
    fn from_path_test() {
        let config = before("from_path_test");
        let paths = lib::path::retrieve(&config.tmp_dir).expect("fail to get paths");
        let (mut files, errors) = from_paths(&paths);
        files.sort();

        assert_eq!(
            files[0].template,
            Template {
                aws_template_format_version: String::from("2010-09-09")
            }
        );
        assert_eq!(errors.len(), 2);
    }
}
