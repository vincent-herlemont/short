//! Inspection and manipulation of cloudformation templates.
use std::path::PathBuf;
use std::error::Error;
use std::fs;
use crate::path;
use serde::Deserialize;
use serde::export::fmt::Debug;

static _TEMPLATE_VERSION: &str = "2010-09-09";
static YAML_EXTENSIONS: [&str; 2] = ["yaml","yml"];

/// The configuration of the cloudformation inspector.
#[derive(Debug)]
struct InspectorConfig {
    path:PathBuf,
}

/// File system information
/// TODO: move to another module
#[derive(Debug)]
struct File {
    path:PathBuf,
    contents:String,
    template:Template,
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
fn from_paths(paths:&[PathBuf]) -> Vec<Result<File,Box<dyn Error>>> {

    // Filter extensions
    let paths = path::filter_extensions(&paths,&YAML_EXTENSIONS);

    // Read contents
    paths.into_iter().filter_map(|path| {
        if let Ok(contents) = fs::read_to_string(&path) {
            return match serde_yaml::from_str(contents.as_str()) {
                Ok(template) => {
                    let file = File {
                        path,
                        contents,
                        template,
                    };
                    Some(Ok(file))
                },
                Err(error) => {
                    Some(Err(Box::new(error) as _))
                }
            }
        }
        None
    }).collect::<Vec<Result<File, Box<_>>>>()
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use crate::cloudformation::{InspectorConfig, from_paths};
    use crate::resource;
    use std::fs;
    use crate::path;

    /// Return [`InspectorConfig`], create temporary directory and copy resource on it.
    ///
    /// The temporary directory is owned by [`InspectorConfig.path`].
    /// And will be deleted when [`InspectorConfig.path`] will be dropped.
    ///
    /// # Recommendation
    ///
    /// Need to call [`after`] at the end of test.
    fn before(test_name: &str) -> InspectorConfig {
        let test_name = format!("{}.{}","d4d",test_name);

        // Create temporary directory.
        let path = TempDir::new(test_name.as_str()).expect("fail to create temporary directory").into_path();

        // Copy resources to it.
        resource::to_dir(&path).expect("fail to copy resources");

        InspectorConfig {
            path
        }
    }

    /// Clean test
    ///
    /// # Recommendation
    ///
    /// Need to be call at the end of the test who use [`before`].
    fn after(cfg:InspectorConfig) {
        // Clean tmp directory
        fs::remove_dir_all(cfg.path).expect("can not clean tmp directory");
    }

    #[test]
    fn from_path_test() {
        let cfg = before("from_path_test");
        let paths = path::retrieve(&cfg.path).expect("fail to get paths");
        let results = from_paths(&paths);
        let result = results[0].as_ref().ok().unwrap();
        assert_eq!(result.template.aws_template_format_version,String::from("2010-09-09"));
        after(cfg);
    }
}

