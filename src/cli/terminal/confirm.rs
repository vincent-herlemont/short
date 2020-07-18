use colored::*;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::fmt::Write;
use std::io;
use std::string::ToString;

use anyhow::{Result};
use serde::export::fmt::Debug;


use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::process::exit;

pub fn confirm<W, E>(mut writer: W, question: &str, e: Vec<E>) -> Result<E>
where
    W: io::Write,
    E: EnumConfirm + Clone + Debug,
{
    let mut write_question_line = || writeln!(writer, "{} : {}", question, &e.to_string()).unwrap();

    write_question_line();
    enable_raw_mode()?;
    let e = loop {
        if let Event::Key(event) = read()? {
            if event == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)
                || event == KeyEvent::new(KeyCode::Char('z'), KeyModifiers::CONTROL)
            {
                disable_raw_mode()?;
                exit(1);
            }

            if let Some(e) = e.iter().find_map(|e| {
                if event == KeyCode::Char(e.to_char()).into() {
                    Some(e)
                } else {
                    None
                }
            }) {
                break e;
            } else {
                disable_raw_mode()?;
                write_question_line();
                enable_raw_mode()?;
            }
        }
    };
    disable_raw_mode()?;
    Ok(e.clone())
}

pub trait EnumConfirm {
    type T: EnumConfirm + Sized;
    fn to_vec() -> Vec<Self::T>;
    fn to_char(&self) -> char;
}

pub trait ToStringEnumConfirm {
    fn to_string(&self) -> String;
}

impl<E> ToStringEnumConfirm for Vec<E>
where
    E: EnumConfirm,
{
    fn to_string(&self) -> String {
        let mut buf = String::new();
        write!(&mut buf, "[").unwrap();
        for (i, e) in self.iter().enumerate() {
            if i > 0 {
                write!(&mut buf, ",").unwrap();
            }
            write!(&mut buf, "{}", e.to_char().to_string().bold()).unwrap();
        }
        write!(&mut buf, "]").unwrap();
        buf
    }
}

#[macro_export]
macro_rules! enum_confirm {
    ($i :ident, $($it: ident), +) => {
        #[derive(Debug, Eq , PartialEq, Clone)]
        pub enum $i {
            $(
                #[allow(non_camel_case_types)]
                $it,
            )+
        }
        impl EnumConfirm for $i {
            type T = Self;

            fn to_vec() -> Vec<Self::T> {
                vec![$( $i::$it, )+]
            }

            fn to_char(&self) -> char {
                match self {
                    $( $i::$it => stringify!($it).chars().next().unwrap(), )+
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::cli::terminal::confirm::ToStringEnumConfirm;
    use crate::cli::terminal::confirm::{EnumConfirm};

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
        assert_eq!(actual.to_string(), "[y,Y,n]".to_string());
    }
}
