use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

pub type Result<T> = std::result::Result<T, Error>;

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

    fn tuple(&self) -> (String, String) {
        (self.name.to_owned(), self.value.to_owned())
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
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    /// ```
    /// use env::Env;
    /// let mut env = Env::new();
    ///
    /// env.add("var1","test");
    ///
    /// if let Some((_,value)) = env.get("var1") {
    ///     assert!(true);
    /// } else {
    ///     assert!(false);
    /// }
    ///
    /// ```
    pub fn add<N, V>(&mut self, name: N, value: V)
    where
        N: AsRef<str>,
        V: AsRef<str>,
    {
        let name = String::from(name.as_ref());
        let value = String::from(value.as_ref());
        let entry = Entry::Var(Var { name, value });
        self.entries.append(&mut vec![entry])
    }

    /// ```
    /// use env::Env;
    /// let mut env = Env::new();
    ///
    /// env.add("var1","test");
    ///
    /// if let Some((_,value)) = env.get("var1") {
    ///     assert!(true);
    /// } else {
    ///     assert!(false);
    /// }
    ///
    /// assert!(env.get("var2").is_none());
    ///
    /// ```
    pub fn get<N: AsRef<str>>(&self, name: N) -> Option<(String, String)> {
        self.entries.iter().find_map(|entry| {
            if let Entry::Var(var) = entry {
                if var.name == String::from(name.as_ref()) {
                    return Some(var.tuple());
                }
            }
            None
        })
    }

    pub fn add_empty_line(&mut self) {
        self.entries.append(&mut vec![Entry::Empty]);
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = OpenOptions::new().read(true).open(path.as_ref())?;
        let mut buf_reader = BufReader::new(file);
        Env::from_reader(&mut buf_reader)
    }

    pub fn from_reader(cursor: &mut dyn BufRead) -> Result<Self> {
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

    pub fn iter(&self) -> EnvIterator {
        EnvIterator {
            index: 0,
            env: &self,
        }
    }
}

#[derive(Debug)]
pub struct EnvIterator<'a> {
    env: &'a Env,
    index: usize,
}

impl<'a> Iterator for EnvIterator<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(var) = self.env.entries.get(self.index) {
            self.index += 1;
            if let Entry::Var(var) = var {
                return Some(var.tuple());
            } else {
                return self.next();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::Env;

    #[test]
    fn env_iterator() {
        let mut env = Env::new();
        env.add("name1", "value1");
        env.add_empty_line();
        env.add("name2", "value2");

        let mut iter = env.iter();

        if let Some((name, value)) = iter.next() {
            assert_eq!(name, "name1");
            assert_eq!(value, "value1");
        } else {
            assert!(false);
        }

        if let Some((name, value)) = iter.next() {
            assert_eq!(name, "name2");
            assert_eq!(value, "value2");
        } else {
            assert!(false);
        }

        assert!(iter.next().is_none());
    }
}
