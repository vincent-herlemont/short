//! D4d specific Error type
//! Allow to embedded multiple error type.
/// Inspiration from : https://github.com/brson/basic-http-server/blob/1ab052719a88e41822b2955d7d72bf161457d47c/src/main.rs#L468
use serde::export::fmt::Debug;
use std::error::Error as StdError;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::io;

// TODO: Add PartialEq to Error type.
#[derive(Debug)]
pub enum Error {
    Other(String),
    Io(io::Error),
}

impl Error {
    /// Create new error
    /// # Example
    /// Usually use [`format!`] as string parameter for pass context information.
    ///
    /// ```
    /// use utils::error::Error;
    /// let url = "http://.....";
    /// Error::new(format!("fail to get url {}",url));
    /// ```
    pub fn new(msg: String) -> Error {
        Error::Other(msg)
    }

    /// Create new box error
    pub fn new_box(msg: String) -> Box<Error> {
        Box::new(Error::new(msg))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;

        match self {
            Io(e) => Some(e),
            Other(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use crate::result::Result;
    use std::io;

    #[test]
    fn error_test() {
        fn create_io_error() -> Result<()> {
            Err(Error::from(io::Error::new(io::ErrorKind::Other, "ho no !")))
        }
        fn create_other_error() -> Result<()> {
            Err(Error::new(format!("ho no !")))
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
    }
}
