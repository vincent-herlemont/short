use crate::utils::write_all::write_all_dir;
use fs_extra;
pub use read_dir::read_dir;
use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::{Path, PathBuf};
use thiserror::Error;

mod read_dir;

pub type Result<T> = std::result::Result<T, EnvError>;
pub type ResultParse<T> = std::result::Result<T, EnvReaderError>;

#[derive(Error, Debug)]
pub enum EnvReaderError {
    #[error("io env reader error")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("space on var name `{0}`")]
    SpaceOnVarName(String),
    #[error("unknown env error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum EnvError {
    #[error("io env error")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("fs_extra env error")]
    FsExtra {
        #[from]
        source: fs_extra::error::Error,
    },
    #[error("fail to parse `{file:?}`")]
    FailToParse {
        #[source]
        source: EnvReaderError,
        file: PathBuf,
    },
    #[error("env var `{0}` not found in `{1:?}`")]
    EnvVarNotFound(String, PathBuf),
    #[error("env file `{0:?}` has no file name")]
    EnvFileHasNoFileName(PathBuf),
    #[error("env file `{0:?}` has an empty file name")]
    EnvFileNameIsEmpty(PathBuf),
    #[error("env file `{0:?}` has incorrect file name : it must begin with `.` char")]
    EnvFileNameIncorrect(PathBuf),
}

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

    fn from_line(line: &String) -> ResultParse<Self> {
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

#[derive(Debug)]
pub struct Env {
    file: PathBuf,
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
    pub fn new(file: PathBuf) -> Self {
        Self {
            file,
            entries: vec![],
        }
    }

    /// ```
    /// use crate::short::env_file::Env;
    /// let mut env = Env::new("".into());
    ///
    /// env.add("var1","test");
    ///
    /// if let Ok((_,value)) = env.get("var1") {
    ///     assert!(true);
    /// } else {
    ///     assert!(false);
    /// }
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
    /// use crate::short::env_file::Env;
    /// let mut env = Env::new("".into());
    ///
    /// env.add("var1","test");
    ///
    /// if let Ok((_,value)) = env.get("var1") {
    ///     assert!(true);
    /// } else {
    ///     assert!(false);
    /// }
    /// assert!(env.get("var2").is_err());
    /// ```
    pub fn get<N: AsRef<str>>(&self, name: N) -> Result<(String, String)> {
        self.entries
            .iter()
            .find_map(|entry| {
                if let Entry::Var(var) = entry {
                    if var.name == String::from(name.as_ref()) {
                        return Some(var.tuple());
                    }
                }
                None
            })
            .ok_or(EnvError::EnvVarNotFound(
                name.as_ref().to_owned(),
                self.file.clone(),
            ))
    }

    pub fn is_set<N, V>(&self, name: N, value: V) -> bool
    where
        N: AsRef<str>,
        V: AsRef<str>,
    {
        self.get(name)
            .map_or(false, |(_, env_value)| env_value == value.as_ref())
    }

    pub fn add_empty_line(&mut self) {
        self.entries.append(&mut vec![Entry::Empty]);
    }

    pub fn from_env_name_reader<P: AsRef<Path>>(path: P, env_name: &String) -> Result<Self> {
        let file = path_from_env_name(path, env_name);
        Self::from_file_reader(file)
    }

    pub fn from_file_reader<P: AsRef<Path>>(file: P) -> Result<Self> {
        let file = file.as_ref().to_path_buf();
        let concrete_file = OpenOptions::new().read(true).open(&file)?;
        let mut buf_reader = BufReader::new(concrete_file);
        let mut env = Env::new(file.clone());
        env.entries_from_reader(&mut buf_reader)
            .map_err(|err| EnvError::FailToParse {
                source: err,
                file: file,
            })?;
        Ok(env)
    }

    pub fn entries_from_reader(&mut self, cursor: &mut dyn BufRead) -> ResultParse<()> {
        for line in cursor.lines() {
            let line = line.map_err(|err| EnvReaderError::Io { source: err })?;
            let line = line.trim_start();
            let line = line.trim_end();
            if line.len() <= 0 {
                let empty = Entry::Empty;
                self.entries.append(&mut vec![empty]);
            } else if let Some(comment) = Comment::from_line(&line.to_string()) {
                let comment = Entry::Comment(comment);
                self.entries.append(&mut vec![comment]);
            } else {
                let line = Var::from_line(&line.to_string())?;
                let var = Entry::Var(line);
                self.entries.append(&mut vec![var]);
            }
        }
        Ok(())
    }

    pub fn set_file(&mut self, file: PathBuf) {
        self.file = file;
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn file_name(&self) -> Result<String> {
        if let Some(file_name) = self.file.file_name() {
            let file_name = file_name.to_str().unwrap().to_string();
            Ok(file_name)
        } else {
            Err(EnvError::EnvFileHasNoFileName(self.file.to_owned()))
        }
    }

    pub fn name(&self) -> Result<String> {
        let file_name = self.file_name()?;
        if file_name
            .chars()
            .next()
            .ok_or(EnvError::EnvFileNameIsEmpty(self.file.clone()))?
            != '.'
        {
            return Err(EnvError::EnvFileNameIncorrect(self.file.clone()));
        }
        let name = file_name.trim_start_matches('.');
        return Ok(name.to_string());
    }

    pub fn iter(&self) -> EnvIterator {
        EnvIterator {
            index: 0,
            env: &self,
        }
    }

    pub fn save(&self) -> Result<()> {
        let content = self.to_string();
        write_all_dir(&self.file, content)?;
        Ok(())
    }
}

impl From<PathBuf> for Env {
    fn from(file: PathBuf) -> Self {
        Self {
            file,
            entries: vec![],
        }
    }
}

pub fn path_from_env_name<P: AsRef<Path>>(dir: P, env_name: &String) -> PathBuf {
    dir.as_ref()
        .to_path_buf()
        .join(PathBuf::from(format!(".{}", env_name)))
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
    use crate::env_file::Env;
    use std::io::Cursor;
    use std::path::PathBuf;

    #[test]
    fn env_iterator() {
        let mut env = Env::new("".into());
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

    #[test]
    fn name_from() {
        let file = PathBuf::from("/test-env");
        let env: Env = file.into();
        let file_name = env.file_name().unwrap();
        assert_eq!(file_name, "test-env");
    }

    #[test]
    fn name() {
        let mut env = Env::new("/test-env".into());
        assert!(env.name().is_err());
        let file_name = env.file_name().unwrap();
        assert_eq!(file_name, "test-env");
        env.set_file("/.test-env".into());
        let name = env.name().unwrap();
        assert_eq!(name, "test-env");

        // trim dot
        let mut env = Env::new("test/.test-env".into());
        let file_name = env.file_name().unwrap();
        assert_eq!(file_name, ".test-env");
        let name = env.name().unwrap();
        assert_eq!(name, "test-env");
    }

    #[test]
    fn is_set() {
        let mut env = Env::new("".into());
        env.add("name1", "value1");
        let is_set = env.is_set("name1", "value1");
        assert!(is_set);
        let is_set = env.is_set("name1", "value2");
        assert!(!is_set);
    }

    #[test]
    fn empty() {
        let mut content = Cursor::new(br#""#);
        let mut env = Env::new("".into());
        env.entries_from_reader(&mut content).unwrap();
        assert_eq!(format!("{}", env), "")
    }

    #[test]
    fn once_var() {
        let mut content = Cursor::new(br#"A=a"#);
        let mut env = Env::new("".into());
        env.entries_from_reader(&mut content).unwrap();
        assert_eq!(format!("{}", env), "A=a\n")
    }

    #[test]
    fn name_end_with_space() {
        let mut content = Cursor::new(br#"A=a "#);
        let mut env = Env::new("".into());
        env.entries_from_reader(&mut content).unwrap();
        assert_eq!(format!("{}", env), "A=a\n")
    }

    #[test]
    fn name_start_with_space() {
        let mut content = Cursor::new(br#"A= a"#);
        let mut env = Env::new("".into());
        env.entries_from_reader(&mut content).unwrap();
        assert_eq!(format!("{}", env), "A=a\n")
    }

    #[test]
    fn value_end_with_space() {
        let mut content = Cursor::new(br#"A =a"#);
        let mut env = Env::new("".into());
        env.entries_from_reader(&mut content).unwrap();
        assert_eq!(format!("{}", env), "A=a\n");
    }

    #[test]
    fn value_start_with_space() {
        let mut content = Cursor::new(br#" A=a"#);
        let mut env = Env::new("".into());
        env.entries_from_reader(&mut content).unwrap();
        assert_eq!(format!("{}", env), "A=a\n");
    }

    #[test]
    fn value_with_space_inside() {
        let mut content = Cursor::new(br#"A B=a"#);
        let mut env = Env::new("".into());
        let r = env.entries_from_reader(&mut content);
        assert!(r.is_err());
    }

    #[test]
    fn empty_comment() {
        let mut content = Cursor::new(br#"#"#);
        let mut env = Env::new("".into());
        env.entries_from_reader(&mut content).unwrap();
        assert_eq!(format!("{}", env), "#\n")
    }

    #[test]
    fn comment() {
        let mut content = Cursor::new(br#"#test"#);
        let mut env = Env::new("".into());
        env.entries_from_reader(&mut content).unwrap();
        assert_eq!(format!("{}", env), "#test\n")
    }

    #[test]
    fn multi_var() {
        let mut content = Cursor::new(
            br#"A=a
    B=b"#,
        );
        let mut env = Env::new("".into());
        env.entries_from_reader(&mut content).unwrap();
        assert_eq!(format!("{}", env), "A=a\nB=b\n")
    }

    #[test]
    fn multi_var_and_comment() {
        let mut content = Cursor::new(
            br#"A=a
#test
B=b"#,
        );
        let mut env = Env::new("".into());
        env.entries_from_reader(&mut content).unwrap();
        assert_eq!(format!("{}", env), "A=a\n#test\nB=b\n")
    }

    #[test]
    fn empty_lines() {
        let mut content = Cursor::new(
            br#"

"#,
        );
        let mut env = Env::new("".into());
        env.entries_from_reader(&mut content).unwrap();
        assert_eq!(format!("{}", env), "\n\n")
    }
}
