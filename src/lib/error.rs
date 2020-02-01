//! D4d specific type of error

use serde::export::fmt::Debug;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct D4dError {
    pub detail: String,
}

impl D4dError {
    /// Create new error
    /// # Example
    /// Usually use [`format!`] as string parameter for pass context information.
    ///
    /// ```
    /// let url = "http://.....";
    /// D4dError::new(format!("fail to get url",url);
    /// ```
    pub fn new(msg: String) -> D4dError {
        D4dError { detail: msg }
    }

    /// Create new box error
    pub fn new_box(msg: String) -> Box<D4dError> {
        Box::new(D4dError::new(msg))
    }
}

impl Display for D4dError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.detail)
    }
}

impl Error for D4dError {}
