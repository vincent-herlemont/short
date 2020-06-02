use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CfgError {
    #[error("setup not found `{0}`")]
    SetupNotFound(String),
    #[error("project not found `{0:?}`")]
    ProjectNotFound(PathBuf),
    #[error("project `{0:?}` already added")]
    ProjectAlreadyAdded(PathBuf),
    #[error("private env dir not found for `{0}`")]
    PrivateEnvDirNotFound(String),
    #[error("private env dir must be an absolute path `{0:?}` for `{1}`")]
    PrivateEnvDirRelativePath(PathBuf, String),
    #[error("public env dir not found for `{0}`")]
    PublicEnvDirNotFound(String),
    #[error("public env dir already unset for `{0}`")]
    PublicEnvAlreadyUnset(String),
    #[error("private env dir already unset for `{0}`")]
    PrivateEnvAlreadyUnset(String),
    #[error("unknown cfg error")]
    Unknown,
}
