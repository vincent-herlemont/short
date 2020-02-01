//! Input/Ouput manipulation operations related of d4d domain.
use crate::d4d_error;

use std::error::Error;
use std::io::BufRead;

/// Search for a buffer a line that satisfies a predicate
/// Return [`String`] that hold the contain of buffer.
pub fn read_to_string_finds<B, F>(mut buf_read: B, f: F) -> Result<String, Box<dyn Error>>
where
    B: BufRead,
    F: Fn(&str) -> bool,
{
    let mut buf = String::new();
    let mut matched = false;
    while buf_read.read_line(&mut buf)? > 0 as usize {
        if !matched {
            matched = f((&buf).as_str())
        }
    }
    match matched {
        true => Ok(buf),
        false => Err(Box::new(d4d_error::new("fail to match"))),
    }
}

#[cfg(test)]
mod tests {
    use crate::lib;
    use std::io::Cursor;

    #[test]
    fn read_to_string_finds_test() {
        let data = b"line1\nline2\n\ntest";
        let cursor = Cursor::new(data);
        let contents = lib::io::read_to_string_finds(cursor, |s| s.contains("test")).unwrap();
        assert_eq!(contents.into_bytes(), data);
    }

    #[test]
    fn read_to_string_finds_test_fail() {
        let data = b"line1\nline2\n\ntest";
        let cursor = Cursor::new(data);
        let result = lib::io::read_to_string_finds(cursor, |s| s.contains("notfouned"));
        assert!(result.is_err());
    }
}
