pub mod aws;
pub mod output;

use crate::exec::output::Output;
use serde::export::Formatter;
use std::fmt;
use std::fmt::Display;
use std::fmt::Write as FmtWrite;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

use utils::error::Error;
use utils::result::Result;
use which;

#[derive(Debug)]
pub struct Software<'s> {
    path: PathBuf,
    args: Vec<String>,
    exec_ctx: &'s ExecCtx,
}

#[derive(Debug)]
pub struct EmptyCtx {}

pub struct Runner<'s, C> {
    path: PathBuf,
    args: Vec<String>,
    ctx: C,
    exec_ctx: &'s ExecCtx,
    display: Box<dyn Fn(String) -> () + Send + Sync>,
}

impl<'s, C> Runner<'s, C> {
    pub fn command(&self) -> Result<Command> {
        let mut command = Command::new(
            self.path
                .to_str()
                .ok_or(format!(
                    "forbidden no UTF-8 to path {}",
                    self.path.to_string_lossy()
                ))?
                .trim(),
        );
        command.args(self.args.clone());
        Ok(command)
    }

    pub fn exec_ctx(&self) -> &ExecCtx {
        self.exec_ctx
    }

    pub fn set_display<D: Fn(String) -> () + Send + Sync + 'static>(self, display: Box<D>) -> Self {
        Self { display, ..self }
    }

    pub fn output(self) -> Result<Output<C>> {
        let output = self.command()?.output().map_err(|e| Error::from(e))?;
        Ok(Output::new(self.ctx, output))
    }

    pub fn spawn(self) -> Result<()> {
        println!("{}", &self);
        // TODO : See https://github.com/oconnor663/duct.rs
        //      Also print stderr
        if !self.exec_ctx.dry_run() {
            let mut child = self.command()?.stdout(Stdio::piped()).spawn()?;

            {
                let stdout = child
                    .stdout
                    .as_mut()
                    .ok_or(Error::new("fail to read stdout"))?;
                let stdout_reader = BufReader::new(stdout);
                let stdout_lines = stdout_reader.lines();
                for line in stdout_lines {
                    if let Ok(line) = line {
                        println!("{}", line);
                    }
                }
            }
            child.wait()?;
        }
        Ok(())
    }

    pub fn spawn2(self) -> Result<Option<Output<C>>> {
        if !self.exec_ctx.dry_run() {
            let mut child = self
                .command()?
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

            // Collect and print stdout
            let (stdout_tx, stdout_rx) = mpsc::channel();
            let stdout = child.stdout.take().expect("fail to read stdout");
            let display = self.display;
            thread::spawn(move || {
                let buffer = readline_std(stdout, display);
                stdout_tx.send(buffer).unwrap();
            });

            // Collect and print stderr
            let (stderr_tx, stderr_rx) = mpsc::channel();
            let stderr = child.stderr.take().expect("fail to read stderr");
            thread::spawn(move || {
                let buffer = readline_std(stderr, Box::new(|_| ()));
                stderr_tx.send(buffer).unwrap();
            });

            // Tread exit status
            let exit_status = child.wait()?;
            let mut exit_status_error = None;
            if !exit_status.success() {
                exit_status_error = Some(Error::from(exit_status.code()));
            }

            let stdout_received = stdout_rx.recv().unwrap();
            let stderr_received = stderr_rx.recv().unwrap();
            let output = Output {
                ctx: self.ctx,
                stderr: stderr_received.into_bytes(),
                stdout: stdout_received.into_bytes(),
                fail: exit_status_error,
            };
            return Ok(Some(output));
        }
        Ok(None)
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
            if let Some(err) = output.fail {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn run2(self) -> Result<Option<Output<C>>> {
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
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}

fn readline_std<T: Read, F: Fn(String) -> ()>(std: T, output: F) -> String {
    let stderr_reader = BufReader::new(std);
    let stderr_lines = stderr_reader.lines();
    let mut buffer = String::new();
    // Fix: error read line by line and create buffer from it can corrupt the buffer.
    for line in stderr_lines {
        if let Ok(line) = line {
            writeln!(&mut buffer, "{}", line.clone().as_str()).unwrap();
            if !line.is_empty() {
                output(line);
            }
        }
    }
    buffer
}

impl<'s, C> Display for Runner<'s, C> {
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

    pub fn runner<C>(self, ctx: C) -> Runner<'s, C> {
        Runner {
            path: self.path,
            args: self.args,
            ctx: ctx,
            exec_ctx: self.exec_ctx,
            display: Box::new(|line| println!("{}", line)),
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
    verbose: bool,
}

impl ExecCtx {
    pub fn new() -> Self {
        Self {
            dry_run: false,
            verbose: false,
        }
    }

    pub fn set_dry_run(&mut self, dry_run: bool) {
        self.dry_run = dry_run;
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn dry_run(&self) -> bool {
        self.dry_run
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::{EmptyCtx, ExecCtx, Software};

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
        let output = soft.runner(EmptyCtx {}).output();
        assert!(output.is_ok());
    }

    #[test]
    fn display_runner() {
        let exec_ctx = ExecCtx::new();
        let mut soft = Software::new("echo", &exec_ctx).unwrap();
        soft.args(&["a b", "b", ""]);
        let runner = soft.runner(EmptyCtx {});
        assert!(format!("{}", &runner).ends_with("echo \"a b\" b "));
        let output = runner.output().unwrap();
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "a b b \n");
    }

    #[test]
    fn exec_ctx() {
        let mut exec_ctx = ExecCtx::new();
        assert!(!exec_ctx.dry_run());
        exec_ctx.set_dry_run(true);
        assert!(exec_ctx.dry_run());
    }

    #[test]
    fn spawn_command() {
        let exec_ctx = ExecCtx::new();
        let mut soft = Software::new("echo", &exec_ctx).unwrap();
        soft.args(&["a b", "b", ""]);
        let runner = soft.runner(EmptyCtx {});
        let output = runner.spawn();
        assert!(output.is_ok());
    }

    #[test]
    fn spawn2_command() {
        let exec_ctx = ExecCtx::new();
        let mut soft = Software::new("sh", &exec_ctx).unwrap();
        soft.args(&["-c", "echo out1 && echo err 1>&2 && echo out2"]);
        let runner = soft.runner(EmptyCtx {});
        let output = runner.spawn2();
        assert!(output.is_ok());
    }
}
