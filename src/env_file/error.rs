use std::io;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvReaderError {
    #[error("io env reader error")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("space on var name `{0}`")]
    SpaceOnVarName(String),
    #[error("unknown env error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum EnvError {
    #[error("io env error")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("fs_extra env error")]
    FsExtra {
        #[from]
        source: fs_extra::error::Error,
    },
    #[error("fail to parse `{file:?}`")]
    FailToParse {
        #[source]
        source: EnvReaderError,
        file: PathBuf,
    },
    #[error("env var `{0}` not found in `{1:?}`")]
    EnvVarNotFound(String, PathBuf),
    #[error("env file `{0:?}` has no file name")]
    EnvFileHasNoFileName(PathBuf),
    #[error("env file `{0:?}` has an empty file name")]
    EnvFileNameIsEmpty(PathBuf),
    #[error("env file `{0:?}` has incorrect file name : it must begin with `.` char")]
    EnvFileNameIncorrect(PathBuf),
}
