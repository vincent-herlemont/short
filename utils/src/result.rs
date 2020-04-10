//! short specific Result type
/// Inspiration from : https://github.com/brson/basic-http-server/blob/1ab052719a88e41822b2955d7d72bf161457d47c/src/main.rs#L447
use super::error::Error;
use serde::export::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

/// Unwrap valid value and failure else [`panic!`].
/// It's usefull for deal with result returned by [`Iterator::partition(Result::is_ok)`].
///
/// # Warning
///
/// To be careful that results are grouped correctly,
/// like Ok(s) to the left and Err(s) to the right : [`(Vec<Result::Ok>,Vec<Result::Err>)`]
///
/// # Example
/// ```
/// use utils::result::unwrap_partition;
/// use utils::result::Result;
/// use utils::error::Error;
/// let results:Vec<Result<String>> = vec![
///     Ok("oui".to_string()),
///     Err(Error::Other("fail".to_string())),
/// ];
/// let results = results.into_iter().partition(Result::is_ok);
/// let results = unwrap_partition(results);
/// ```
pub fn unwrap_partition<T>((oks, errors): (Vec<Result<T>>, Vec<Result<T>>)) -> (Vec<T>, Vec<Error>)
where
    T: Debug,
{
    let oks = oks.into_iter().map(|ok| ok.unwrap()).collect();
    let errors = errors.into_iter().map(|error| error.unwrap_err()).collect();
    (oks, errors)
}
