
use std::path::PathBuf;
use thiserror::Error;

type SetupName = String;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("`{0:?}` not found for `{1}`")]
    EnvDirNotFound(PathBuf, SetupName, #[source] std::io::Error),
    #[error("open editor fail")]
    OpenEditorFail,
}
