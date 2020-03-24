pub mod aws;

use serde::export::Formatter;
use std::fmt;
use std::fmt::Display;

use std::path::PathBuf;
use std::process::Command;
use utils::error::Error;
use utils::result::Result;
use which;

#[derive(Debug)]
pub struct Software {
    path: PathBuf,
    args: Vec<String>,
}

pub struct Runner {
    path: PathBuf,
    args: Vec<String>,
}

impl Runner {
    pub fn command(self) -> Result<Command> {
        let mut command = Command::new(
            self.path
                .to_str()
                .ok_or(format!(
                    "forbidden no UTF-8 to path {}",
                    self.path.to_string_lossy()
                ))?
                .trim(),
        );
        command.args(self.args);
        Ok(command)
    }

    pub fn output(self) -> Result<std::process::Output> {
        self.command()?.output().map_err(|e| Error::from(e))
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}

impl Display for Runner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())?;
        for arg in &self.args {
            write!(f, " ")?;
            if let Some(_) = arg.find(char::is_whitespace) {
                write!(f, "\"{}\"", arg)?;
            } else {
                write!(f, "{}", arg)?;
            }
        }
        Ok(())
    }
}

impl Software {
    pub fn new<N: AsRef<str>>(name: N) -> Result<Self> {
        let name = String::from(name.as_ref());
        let path = which::which(&name)
            .map_err(|e| Error::wrap(format!("fail to found {} command", &name), Error::from(e)))?;
        Ok(Software { path, args: vec![] })
    }

    pub fn arg<S: AsRef<str>>(&mut self, arg: S) {
        self.args.append(&mut vec![String::from(arg.as_ref())]);
    }

    pub fn args<I>(&mut self, args: I)
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        for arg in args {
            self.arg(arg)
        }
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn runner(self) -> Runner {
        Runner {
            path: self.path,
            args: self.args,
        }
    }

    pub fn path(&self) -> PathBuf {
        self.path.to_owned()
    }

    pub fn fake<N: AsRef<str>>(name: N) -> Self {
        Software {
            path: PathBuf::from(name.as_ref()),
            args: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::Software;

    #[test]
    fn new() {
        let soft = Software::new("rustc");
        assert!(soft.is_ok());
        let soft = Software::new("taratata");
        assert!(soft.is_err());
    }

    #[test]
    fn arg() {
        let mut soft = Software::new("rustc").unwrap();
        soft.arg("--version");
        assert_eq!(soft.get_args(), &vec![String::from("--version")])
    }

    #[test]
    fn args() {
        let mut soft = Software::new("rustc").unwrap();
        soft.args(&["--help", "-v"]);
        assert_eq!(
            soft.get_args(),
            &vec![String::from("--help"), String::from("-v")]
        )
    }

    #[test]
    fn output() {
        let mut soft = Software::new("rustc").unwrap();
        soft.arg("--version");
        let output = soft.runner().output();
        assert!(output.is_ok());
    }

    #[test]
    fn display_runner() {
        let mut soft = Software::new("echo").unwrap();
        soft.args(&["a b", "b", ""]);
        let runner = soft.runner();
        assert_eq!(format!("{}", &runner), "/usr/bin/echo \"a b\" b ");
        let output = runner.output().unwrap();
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "a b b \n");
    }
}
