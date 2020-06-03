use crate::env_file::{EnvReaderError, ResultParse};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Var {
    name: String,
    value: String,
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}={}", self.name, self.value)
    }
}

impl Var {
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: AsRef<str>,
        V: AsRef<str>,
    {
        Self {
            name: String::from(name.as_ref()),
            value: String::from(value.as_ref()),
        }
    }

    pub fn from_line(line: &String) -> ResultParse<Self> {
        let vars: Vec<&str> = line.rsplitn(2, "=").collect();
        match vars.as_slice() {
            [value, name] => {
                let value = value.trim_end();
                let value = value.trim_start();
                let name = name.trim_end();
                let name = name.trim_start();

                if name.contains(char::is_whitespace) {
                    return Err(EnvReaderError::SpaceOnVarName(name.to_owned()));
                }

                Ok(Var::new(name, value))
            }
            _ => Err(EnvReaderError::Unknown),
        }
    }

    pub fn tuple(&self) -> (String, String) {
        (self.name.to_owned(), self.value.to_owned())
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}
