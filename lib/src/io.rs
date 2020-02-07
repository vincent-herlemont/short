//! Input/Ouput manipulation operations related of d4d domain.
use crate::error::Error;
use crate::result::Result;
use std::io::BufRead;

/// TODO: make monade of [`ContainStr`] function.
/// --- like ~= type ContainStr = Fn(&str) -> bool;

/// Search for a buffer a line that satisfies a predicate
/// Return [`String`] that hold the contain of buffer.
pub fn read_to_string_contain<B, F>(mut buf_read: B, next_line: F) -> Result<String>
where
    B: BufRead,
    F: Fn(&str) -> bool,
{
    let mut buf = String::new();
    let mut matched = false;
    while buf_read.read_line(&mut buf)? > 0 as usize {
        if !matched {
            matched = next_line((&buf).as_str())
        }
    }
    match matched {
        true => Ok(buf),
        false => Err(Error::new(format!("fail to satisfy predicate"))),
    }
}

#[cfg(test)]
mod tests {
    use crate::io::read_to_string_contain;
    use std::io::Cursor;

    #[test]
    fn read_to_string_finds_test() {
        let data = b"line1\nline2\n\ntest";
        let cursor = Cursor::new(data);
        let contents = read_to_string_contain(cursor, |s| s.contains("test")).unwrap();
        assert_eq!(contents.into_bytes(), data);
    }

    #[test]
    fn read_to_string_finds_test_fail() {
        let data = b"line1\nline2\n\ntest";
        let cursor = Cursor::new(data);
        let result = read_to_string_contain(cursor, |s| s.contains("notfouned"));
        assert!(result.is_err());
    }
}
