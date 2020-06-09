use std::path::PathBuf;
use thiserror::Error;

type SetupName = String;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("`{0:?}` not found for `{1}`")]
    EnvDirNotFound(PathBuf, SetupName, #[source] std::io::Error),
    #[error("open editor fail")]
    OpenEditorFail,
    #[error("bad input `{0}` try again")]
    ConfirmBadInputTryAgain(String),
    #[error("you have not allowed to delete var `{0}`:`{1}` in {2}")]
    DeleteVarNowAllowed(String, String, String),
    #[error("env must be sync, please change it manually or run \"short env sync\"")]
    EnvFileMustBeSync,
    #[error("env file `0:?` already exist")]
    EnvFileAlreadyExist(PathBuf),
}
