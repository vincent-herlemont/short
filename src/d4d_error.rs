//! D4d specific type of error

#[derive(Debug)]
pub struct D4dError {
    pub detail:String,
}

pub fn new(msg:&str) -> D4dError {
    D4dError{detail:msg.to_string()}
}

use std::fmt::{Display, Formatter, Result};

impl Display for D4dError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"{}",self.detail)
    }
}

use std::error::Error;

impl Error for D4dError {}