pub mod aws;

use serde::export::Formatter;
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Output};
use utils::error::Error;
use utils::result::Result;
use which;

#[derive(Debug)]
pub struct Software<'s> {
    path: PathBuf,
    args: Vec<String>,
    exec_ctx: &'s ExecCtx,
}

pub struct Runner<'s> {
    path: PathBuf,
    args: Vec<String>,
    exec_ctx: &'s ExecCtx,
}

impl<'s> Runner<'s> {
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

    pub fn output(self) -> Result<Output> {
        let output = self.command()?.output().map_err(|e| Error::from(e))?;
        Ok(output)
    }

    pub fn run(self) -> Result<()> {
        println!("{}", &self);
        if !self.exec_ctx.dry_run() {
            let output = self.output()?;
            println!(
                "{}",
                String::from_utf8(output.stderr.clone()).expect("fail to read stderr")
            );
            println!(
                "{}",
                String::from_utf8(output.stdout.clone()).expect("fail to read stdout")
            );
            println!("{}", output.status);
            if output.status.success() {
                return Ok(());
            } else {
                return Err(Error::from(output.status));
            }
        }
        Ok(())
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}

impl<'s> Display for Runner<'s> {
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

impl<'s> Software<'s> {
    pub fn new<N: AsRef<str>>(name: N, exec_ctx: &'s ExecCtx) -> Result<Self> {
        let name = String::from(name.as_ref());
        let path = if !exec_ctx.dry_run() {
            which::which(&name).map_err(|e| {
                Error::wrap(format!("fail to found {} command", &name), Error::from(e))
            })?
        } else {
            PathBuf::from(name)
        };
        Ok(Software {
            path,
            args: vec![],
            exec_ctx,
        })
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

    pub fn runner(self) -> Runner<'s> {
        Runner {
            path: self.path,
            args: self.args,
            exec_ctx: self.exec_ctx,
        }
    }

    pub fn path(&self) -> PathBuf {
        self.path.to_owned()
    }

    pub fn fake<N: AsRef<str>>(name: N, exec_ctx: &'s ExecCtx) -> Self {
        Software {
            path: PathBuf::from(name.as_ref()),
            args: vec![],
            exec_ctx,
        }
    }
}

#[derive(Debug)]
pub struct ExecCtx {
    dry_run: bool,
}

impl ExecCtx {
    pub fn new() -> Self {
        Self { dry_run: false }
    }

    pub fn set_dry_run(self, dry_run: bool) -> Self {
        Self { dry_run }
    }

    pub fn dry_run(&self) -> bool {
        self.dry_run
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::{ExecCtx, Software};

    #[test]
    fn new() {
        let exec_ctx = ExecCtx::new();
        let soft = Software::new("rustc", &exec_ctx);
        assert!(soft.is_ok());
        let soft = Software::new("taratata", &exec_ctx);
        assert!(soft.is_err());
    }

    #[test]
    fn arg() {
        let exec_ctx = ExecCtx::new();
        let mut soft = Software::new("rustc", &exec_ctx).unwrap();
        soft.arg("--version");
        assert_eq!(soft.get_args(), &vec![String::from("--version")])
    }

    #[test]
    fn args() {
        let exec_ctx = ExecCtx::new();
        let mut soft = Software::new("rustc", &exec_ctx).unwrap();
        soft.args(&["--help", "-v"]);
        assert_eq!(
            soft.get_args(),
            &vec![String::from("--help"), String::from("-v")]
        )
    }

    #[test]
    fn output() {
        let exec_ctx = ExecCtx::new();
        let mut soft = Software::new("rustc", &exec_ctx).unwrap();
        soft.arg("--version");
        let output = soft.runner().output();
        assert!(output.is_ok());
    }

    #[test]
    fn display_runner() {
        let exec_ctx = ExecCtx::new();
        let mut soft = Software::new("echo", &exec_ctx).unwrap();
        soft.args(&["a b", "b", ""]);
        let runner = soft.runner();
        assert!(format!("{}", &runner).ends_with("echo \"a b\" b "));
        let output = runner.output().unwrap();
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "a b b \n");
    }

    #[test]
    fn exec_ctx() {
        let exec_ctx = ExecCtx::new();
        assert!(!exec_ctx.dry_run());
        let exec_ctx = exec_ctx.set_dry_run(true);
        assert!(exec_ctx.dry_run());
    }
}
