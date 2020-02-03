//! D4d specific Result type
/// Inspiration from : https://github.com/brson/basic-http-server/blob/1ab052719a88e41822b2955d7d72bf161457d47c/src/main.rs#L447
use crate::lib::error::Error;

pub type Result<T> = std::result::Result<T, Error>;
