//! Inspection and manipulation of cloudformation templates.
use serde::export::fmt::Debug;
use serde::Deserialize;
use std::cmp::Ordering;
use std::path::PathBuf;
use utils::error::Error;
use utils::fs::ContentFile;
use utils::path::filter_extensions;
use utils::result::unwrap_partition;

#[allow(dead_code)]
static YAML_EXTENSIONS: [&str; 2] = ["yaml", "yml"];
#[allow(dead_code)]
static TEMPLATE_VERSION: &str = "2010-09-09";

/// The configuration of the cloudformation inspector.
#[derive(Debug)]
struct InspectorConfig {
    path: PathBuf,
}

/// File system information
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
/// Return [`Vec`] of [`Error::Io`] and/or [`Error::SerdeYaml`].
#[allow(dead_code)]
fn from_paths(paths: &[PathBuf]) -> (Vec<File>, Vec<Error>) {
    let paths = filter_extensions(&paths, &YAML_EXTENSIONS);

    let (content_files, mut errors) =
        ContentFile::read_contain_multi(&paths, |line| line.contains(TEMPLATE_VERSION));

    let results: (Vec<_>, Vec<_>) = content_files
        .into_iter()
        .map(
            |content_file| match serde_yaml::from_str::<Template>(&content_file.contents) {
                Ok(template) => Ok(File {
                    content: content_file,
                    template: template,
                }),
                Err(e) => Err(Error::from(e)),
            },
        )
        .partition(Result::is_ok);

    let (files, mut error_files) = unwrap_partition(results);

    errors.append(&mut error_files);

    (files, errors)
}

#[cfg(test)]
mod tests {
    use crate::assets::get_assets;
    use crate::cloudformation::{from_paths, File, Template};
    use utils::assert_find;
    use utils::assert_not_find;
    use utils::error::Error;
    use utils::path::retrieve;
    use utils::test::before;

    #[allow(unreachable_patterns)]
    #[test]
    fn from_path_test() {
        let config = before("from_path_test", &get_assets());
        let paths = retrieve(&config.tmp_dir).expect("fail to get paths");
        let (files, errors) = from_paths(&paths);
        assert_find!(files,File{template,..},
            template == &Template {
            aws_template_format_version: String::from("2010-09-09")
        });
        assert_eq!(errors.len(), 2);
        assert_find!(errors, Error::Io(_));
        assert_find!(errors, Error::SerdeYaml(_));
        assert_not_find!(errors, Error::Other(_));
    }
}
