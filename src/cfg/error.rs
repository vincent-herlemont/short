use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CfgError {
    #[error("setup not found `{0}`")]
    SetupNotFound(String),
    #[error("project not found `{0:?}`")]
    ProjectNotFound(PathBuf),
    #[error("project `{0}` already added")]
    ProjectAlreadyAdded(PathBuf),
    #[error("unknown cfg error")]
    Unknown,
}
