use std::fmt::{Display, Formatter};
/// https://smartmob-rfc.readthedocs.io/en/latest/2-dotenv.html
use std::io::{BufRead, Error, ErrorKind};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Var {
    name: String,
    value: String,
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

impl Var {
    fn new<N, V>(name: N, value: V) -> Self
    where
        N: AsRef<str>,
        V: AsRef<str>,
    {
        Self {
            name: String::from(name.as_ref()),
            value: String::from(value.as_ref()),
        }
    }

    fn from_line(line: &String) -> Result<Self> {
        let vars: Vec<&str> = line.rsplitn(2, "=").collect();
        match vars.as_slice() {
            [_, name] if name.find(char::is_whitespace).map_or(false, |_| true) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("space on name \"{}\"", name),
            )),
            [value, name] => {
                let value = value.trim_end();
                let value = value.trim_start();
                Ok(Var::new(name, value))
            }
            _ => Err(Error::new(ErrorKind::InvalidData, "fail to parse env")),
        }
    }
}

#[derive(Debug)]
pub struct Comment {
    value: String,
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.value)
    }
}

impl Comment {
    fn from_line(line: &String) -> Option<Self> {
        let parts: Vec<&str> = line.splitn(2, "#").collect();
        match parts.as_slice() {
            [empty, comment] if empty.is_empty() => Some(Self {
                value: String::from(comment.to_owned()),
            }),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Entry {
    Var(Var),
    Comment(Comment),
    Empty,
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Var(var) => write!(f, "{}", var),
            Entry::Comment(comment) => write!(f, "{}", comment),
            Entry::Empty => writeln!(f, ""),
        }
    }
}

#[derive(Debug)]
pub struct Env {
    entries: Vec<Entry>,
}

impl Display for Env {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for entry in &self.entries {
            write!(f, "{}", entry)?;
        }
        Ok(())
    }
}

impl Env {
    pub fn from<'b>(buf: &'b mut dyn BufRead) -> Result<Self> {
        let mut entries = vec![];

        for line in buf.lines() {
            let line = line?;
            if let Some(comment) = Comment::from_line(&line) {
                entries.insert(0, Entry::Comment(comment))
            } else {
                let var = Var::from_line(&line)?;
                entries.insert(0, Entry::Var(var));
            }
        }

        Ok(Env { entries })
    }
}
