use std::fmt::{Display, Formatter};
use std::io::{BufRead, Error, ErrorKind};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
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
            [value, name] => {
                let value = value.trim_end();
                let value = value.trim_start();
                let name = name.trim_end();
                let name = name.trim_start();

                if name.contains(char::is_whitespace) {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("space on name \"{}\"", name),
                    ));
                }

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
        writeln!(f, "#{}", self.value)
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

impl Entry {
    fn empty(line: &String) -> Option<Entry> {
        let line = line.trim_start();
        let line = line.trim_end();
        if line.len() > 0 {
            None
        } else {
            Some(Entry::Empty)
        }
    }

    fn comment(line: &String) -> Option<Entry> {
        let comment = Comment::from_line(&line)?;
        Some(Entry::Comment(comment))
    }

    fn var(line: &String) -> Result<Entry> {
        let var = Var::from_line(line)?;
        Ok(Entry::Var(var))
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
    pub fn from(cursor: &mut dyn BufRead) -> Result<Self> {
        let mut entries = vec![];
        for line in cursor.lines() {
            let line = line?;
            if let Some(empty) = Entry::empty(&line) {
                entries.append(&mut vec![empty]);
            } else if let Some(comment) = Entry::comment(&line) {
                entries.append(&mut vec![comment]);
            } else {
                let var = Entry::var(&line)?;
                entries.append(&mut vec![var]);
            }
        }

        Ok(Env { entries })
    }
}
