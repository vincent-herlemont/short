use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs::remove_file;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub use comment::Comment;
pub use diff::EnvDiffController;
pub use error::{EnvError, EnvReaderError};
pub use read_dir::read_dir;
pub use var::Var;

use crate::env_file::entry::Entry;
use crate::env_file::iter::EnvIterator;
use crate::utils::write_all::write_all_dir;

mod comment;
mod diff;
mod entry;
mod error;
mod iter;
mod read_dir;
mod recent;
mod var;

pub type Result<T> = std::result::Result<T, EnvError>;
pub type ResultParse<T> = std::result::Result<T, EnvReaderError>;

#[derive(Debug, Clone, Eq)]
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

impl PartialEq for Env {
    fn eq(&self, other: &Self) -> bool {
        self.file.eq(&other.file)
    }
}

impl PartialOrd for Env {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.file.partial_cmp(&other.file)
    }
}

impl Ord for Env {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file.cmp(&other.file)
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
    /// if let Ok(var) = env.get("var1") {
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
        let entry = Entry::Var(Var::new(name, value));
        self.entries.append(&mut vec![entry])
    }

    /// ```
    /// use crate::short::env_file::Env;
    /// let mut env = Env::new("".into());
    ///
    /// env.add("var1","test");
    ///
    /// if let Ok(var) = env.get("var1") {
    ///     assert!(true);
    /// } else {
    ///     assert!(false);
    /// }
    /// assert!(env.get("var2").is_err());
    /// ```
    pub fn get<N: AsRef<str>>(&self, name: N) -> Result<&Var> {
        self.entries
            .iter()
            .find_map(|entry| {
                if let Entry::Var(var) = entry {
                    if var.name() == &String::from(name.as_ref()) {
                        return Some(var);
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
            .map_or(false, |var| var.value() == value.as_ref())
    }

    pub fn add_empty_line(&mut self) {
        self.entries.append(&mut vec![Entry::Empty]);
    }

    pub fn from_file_reader<P: AsRef<Path>>(file: P) -> Result<Self> {
        let file = file.as_ref().to_path_buf();
        let concrete_file = OpenOptions::new().read(true).open(&file)?;
        let mut buf_reader = BufReader::new(concrete_file);
        let mut env = Env::new(file.clone());
        env.entries_from_reader(&mut buf_reader)
            .map_err(|err| EnvError::FailToParse { source: err, file })?;
        Ok(env)
    }

    pub fn entries_from_reader(&mut self, cursor: &mut dyn BufRead) -> ResultParse<()> {
        for line in cursor.lines() {
            let line = line.map_err(|err| EnvReaderError::Io { source: err })?;
            let line = line.trim_start_matches("\u{feff}"); // Ignore BOM
            let line = line.trim_start(); // Ignore start spaces
            let line = line.trim_end(); // Ignore end spaces
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
        EnvIterator::new(&self)
    }

    pub fn save(&self) -> Result<()> {
        let content = self.to_string();
        write_all_dir(&self.file, content)?;
        Ok(())
    }

    // TODO: test
    pub fn remove(&self) -> Result<()> {
        remove_file(&self.file)?;
        Ok(())
    }

    pub fn copy(&self, file: PathBuf) -> Self {
        Self {
            file,
            entries: self.entries.clone(),
        }
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::path::PathBuf;

    use crate::env_file::Env;

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
        let env = Env::new("test/.test-env".into());
        let file_name = env.file_name().unwrap();
        assert_eq!(file_name, ".test-env");
        let name = env.name().unwrap();
        assert_eq!(name, "test-env");
    }

    #[test]
    fn copy() {
        let mut env = Env::new(".file1".into());
        env.add("name1", "value1");
        let env_copy = env.copy(".file2".into());
        assert_eq!(env.name().unwrap(), "file1");
        assert_eq!(env_copy.name().unwrap(), "file2");
        let name1 = env_copy.get("name1").unwrap();
        assert_eq!(name1.value(), "value1");
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
