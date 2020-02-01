//! Inspection and manipulation of cloudformation templates.
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use serde::export::fmt::Debug;
use serde::Deserialize;

use crate::lib;

static _TEMPLATE_VERSION: &str = "2010-09-09";
static YAML_EXTENSIONS: [&str; 2] = ["yaml", "yml"];

/// The configuration of the cloudformation inspector.
#[derive(Debug)]
struct InspectorConfig {
    path: PathBuf,
}

/// File system information
/// TODO: move to another module
#[derive(Debug)]
struct File {
    path: PathBuf,
    contents: String,
    template: Template,
}

/// Template aws file
#[derive(Debug, PartialEq, Deserialize)]
struct Template {
    #[serde(rename(deserialize = "AWSTemplateFormatVersion"))]
    aws_template_format_version: String,
}

/// Return a list of items [`Result<File>`] matching with AWS cloudformation [`Template`]
///
/// # Errors
///
/// Throw [`yaml_serde::Error`] on [`Result`] items.
///
/// TODO: - move read contents logic to another module.
///       - detect aws template as row file before parse with [`yaml_serde`]
///         it will be useful for detect throw [`yaml_serde::Error`] only on
///         AWS cloudformation template.
fn from_paths(paths: &[PathBuf]) -> Vec<Result<File, Box<dyn Error>>> {
    // Filter extensions
    let paths = lib::path::filter_extensions(&paths, &YAML_EXTENSIONS);

    // Read contents
    paths
        .into_iter()
        .filter_map(|path| {
            if let Ok(contents) = fs::read_to_string(&path) {
                return match serde_yaml::from_str(contents.as_str()) {
                    Ok(template) => {
                        let file = File {
                            path,
                            contents,
                            template,
                        };
                        Some(Ok(file))
                    }
                    Err(error) => Some(Err(Box::new(error) as _)),
                };
            }
            None
        })
        .collect::<Vec<Result<File, Box<_>>>>()
}

#[cfg(test)]
mod tests {
    use crate::cloudformation::from_paths;
    use crate::lib;
    use crate::lib::test::{after, before};

    #[test]
    fn from_path_test() {
        let test_path = before("from_path_test");
        let paths = lib::path::retrieve(&test_path).expect("fail to get paths");
        let results = from_paths(&paths);
        let result = results[0].as_ref().ok().unwrap();
        assert_eq!(
            result.template.aws_template_format_version,
            String::from("2010-09-09")
        );
        after(test_path);
    }
}
