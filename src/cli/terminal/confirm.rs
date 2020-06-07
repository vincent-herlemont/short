#[macro_use]
use anyhow::{Context, Result};
use crate::cli::error::CliError;
use serde::export::fmt::Debug;
use std::fmt::Write;

use std::io;
use std::string::ToString;

pub fn confirm<R, W, E>(mut reader: R, mut writer: W, question: &str, e: Vec<E>) -> Result<E>
where
    R: io::BufRead,
    W: io::Write,
    E: EnumConfirm + ToString + Debug,
{
    writeln!(writer, "{} : {}", question, &e.to_string()).unwrap();
    let mut response = String::new();
    reader.read_line(&mut response)?;
    let response = response.trim_end().to_string();
    let e = e.into_iter().find(|e| e.to_string() == response);
    e.context(CliError::ConfirmBadInputTryAgain(response))
}

pub trait EnumConfirm {
    type T: ToString;
    fn to_vec() -> Vec<Self::T>;
}

pub trait ToStringEnumConfirm {
    fn to_string(&self) -> String;
}

impl<E> ToStringEnumConfirm for Vec<E>
where
    E: EnumConfirm + ToString,
{
    fn to_string(&self) -> String {
        let mut buf = String::new();
        write!(&mut buf, "[ ").unwrap();
        for (i, e) in self.iter().enumerate() {
            if i > 0 {
                write!(&mut buf, ",").unwrap();
            }
            write!(&mut buf, " {} ", e.to_string()).unwrap();
        }
        write!(&mut buf, " ]").unwrap();
        buf
    }
}

#[macro_export]
macro_rules! enum_confirm {
    ($i :ident, $($it: ident), +) => {
        #[derive(Debug, Eq , PartialEq)]
        pub enum $i {
            $( $it, )+
        }
        impl std::string::ToString for $i {
            fn to_string(&self) -> std::string::String {
                match self {
                    $( $i::$it => String::from(stringify!($it)), )+
                }
            }
        }
        impl EnumConfirm for $i {
            type T = Self;

            fn to_vec() -> Vec<Self::T> {
                vec![$( $i::$it, )+]
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::cli::terminal::confirm::ToStringEnumConfirm;
    use crate::cli::terminal::confirm::{confirm, EnumConfirm};

    enum_confirm!(EnumConfirmTest, y, Y, n);

    #[test]
    fn enum_confirm_macro() {
        let actual = EnumConfirmTest::to_vec();
        let expected = vec![EnumConfirmTest::y, EnumConfirmTest::Y, EnumConfirmTest::n];
        assert_eq!(actual, expected);
    }

    #[test]
    fn enum_confirm_to_string() {
        let actual = EnumConfirmTest::to_vec();
        assert_eq!(actual.to_string(), "[  y , Y , n  ]".to_string());
    }

    #[test]
    fn test_confirm() {
        let mut input: &[u8] = b"y\n".as_ref();
        let mut output = vec![];
        let list_enum_confirm_test = EnumConfirmTest::to_vec();
        let r = confirm(
            &mut input,
            &mut output,
            "What do you want to do ?",
            list_enum_confirm_test,
        );
        let output = String::from_utf8(output).unwrap();
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(EnumConfirmTest::y, r);
        assert_eq!(
            output,
            "What do you want to do ? : [  y , Y , n  ]\n".to_string()
        )
    }
}
