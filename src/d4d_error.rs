//! D4d specific type of error

use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct D4dError {
    pub detail: String,
}

pub fn new(msg: &str) -> D4dError {
    D4dError {
        detail: msg.to_string(),
    }
}

impl Display for D4dError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.detail)
    }
}

impl Error for D4dError {}
