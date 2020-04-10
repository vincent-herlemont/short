//! short specific Error type
//! Allow to embedded multiple error type.
use super::result::Result;
use promptly::ReadlineError;
/// Inspiration from : https://github.com/brson/basic-http-server/blob/1ab052719a88e41822b2955d7d72bf161457d47c/src/main.rs#L468
use serde::export::fmt::Debug;
use serde_yaml;

use std::error::Error as StdError;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::io;
use std::path::StripPrefixError;

use std::string::FromUtf8Error;

pub type ExitStatusCode = Option<i32>;

// TODO: Add PartialEq to Error type.
#[derive(Debug)]
pub enum Error {
    Other(String),
    Wrap(String, Box<Error>),
    Io(io::Error),
    SerdeYaml(serde_yaml::Error),
    Which(which::Error),
    StripPrefixError(StripPrefixError),
    ExitStatus(ExitStatusCode),
    RustyLine(ReadlineError),
    StringUtf8(FromUtf8Error),
}

impl Error {
    /// Create new error
    /// # Example
    /// Usually use [`format!`] as string parameter for pass context information.
    ///
    /// ```
    /// use short_utils::error::Error;
    /// let url = "http://.....";
    /// Error::new(format!("fail to get url {}",url));
    /// ```
    pub fn new<S: AsRef<str>>(msg: S) -> Error {
        Error::Other(String::from(msg.as_ref()))
    }

    /// Create new box error
    pub fn new_box(msg: String) -> Box<Error> {
        Box::new(Error::new(msg))
    }

    pub fn wrap<S: AsRef<str>>(msg: S, err: Error) -> Error {
        Error::Wrap(String::from(msg.as_ref()), Box::new(err))
    }

    pub fn is<P>(&self, predicate: P) -> bool
    where
        P: Fn(&Error) -> bool,
    {
        match self {
            Error::Wrap(_, err) => err.is(predicate),
            err => predicate(err),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::Other(msg) => write!(f, "{}", msg),
            Error::Wrap(msg, err) => write!(f, "{} : {}", msg, err),
            Error::Io(err) => write!(f, "{}", err),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;

        match self {
            Io(e) => Some(e),
            SerdeYaml(e) => Some(e),
            Which(_) => None,
            Wrap(_, error) => Some(error),
            Other(_) => None,
            StripPrefixError(e) => Some(e),
            ExitStatus(_) => None,
            RustyLine(e) => Some(e),
            StringUtf8(e) => Some(e),
        }
    }
}

impl From<&'static str> for Error {
    fn from(message: &'static str) -> Error {
        Error::Other(String::from(message))
    }
}

impl From<String> for Error {
    fn from(message: String) -> Error {
        Error::Other(message)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Error {
        Error::SerdeYaml(error)
    }
}

impl From<which::Error> for Error {
    fn from(error: which::Error) -> Error {
        Error::Which(error)
    }
}

impl From<StripPrefixError> for Error {
    fn from(error: StripPrefixError) -> Error {
        Error::StripPrefixError(error)
    }
}

impl From<ReadlineError> for Error {
    fn from(error: ReadlineError) -> Error {
        Error::RustyLine(error)
    }
}

impl From<ExitStatusCode> for Error {
    fn from(exit_status: ExitStatusCode) -> Error {
        Error::ExitStatus(exit_status)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error::StringUtf8(error)
    }
}

impl Error {
    pub fn exit_code_eq(&self, code: i32) -> Result<bool> {
        if let Error::ExitStatus(exit_status) = self {
            let cmd_code = exit_status.ok_or(Error::new("Process terminated by signal"))?;
            return Ok(code == cmd_code);
        }
        Err(Error::new("no exit status")).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use crate::result::Result;
    use std::io;
    use std::io::ErrorKind;

    #[test]
    fn error_test() {
        fn create_io_error() -> Result<()> {
            Err(Error::from(io::Error::new(io::ErrorKind::Other, "ho no !")))
        }
        fn create_other_error() -> Result<()> {
            Err(Error::new(format!("ho no !")))
        }
        fn create_other_error_from() -> Result<()> {
            Err(Error::from("ho no !"))
        }

        let result = create_io_error();
        match result.err().unwrap() {
            Error::Io(_) => assert!(true),
            _ => assert!(false),
        };

        let result = create_other_error();
        match result.err().unwrap() {
            Error::Other(_) => assert!(true),
            _ => assert!(false),
        };

        let result = create_other_error_from();
        match result.err().unwrap() {
            Error::Other(_) => assert!(true),
            _ => assert!(false),
        };
    }

    #[test]
    fn nested_error() {
        let err = Error::from(io::Error::new(ErrorKind::NotFound, "not found"));
        let err = Error::wrap("wrap lvl 1", err);
        let err = Error::wrap("wrap lvl 2", err);
        let b = err.is(|err| matches!(err, Error::Io(_)));
        assert!(b);
        let b = err.is(|err| matches!(err, Error::Other(_)));
        assert!(!b);
    }
}
