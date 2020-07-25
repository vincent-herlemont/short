use colored::*;
use std::fmt::Write as FmtWrite;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::{Context, Result};

pub use file::{set_exec_permision, File};
pub use var::{
    generate_array_env_var, generate_env_var, generate_env_vars, var_name, EnvValue, EnvVar,
    ENV_ENVIRONMENT_VAR, ENV_SETUP_VAR,
};

mod file;
mod kind;
mod var;

#[derive(Debug)]
pub struct Output {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

impl Output {
    pub fn new() -> Self {
        Self {
            status: 0,
            stdout: "".into(),
            stderr: "".into(),
        }
    }
}

impl From<process::Output> for Output {
    fn from(output: process::Output) -> Self {
        Self {
            status: output.status.code().map_or(0, |code| code),
            stderr: String::from_utf8_lossy(output.stderr.as_ref()).into_owned(),
            stdout: String::from_utf8_lossy(output.stdout.as_ref()).into_owned(),
        }
    }
}

pub fn run_as_stream(file: &PathBuf, vars: &Vec<EnvVar>, args: &Vec<String>) -> Result<Output> {
    let file = file.canonicalize()?;
    let mut command = Command::new(&file);

    for env_var in vars.iter() {
        command.env(env_var.var().to_env_var(), env_var.env_value().to_string());
    }

    if let Some(parent) = file.parent() {
        command.current_dir(parent);
    }

    let mut child = command
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .args(args)
        .spawn()
        .context(format!("command {} fail", &file.to_string_lossy()))?;

    let mut command_stdin = child.stdin.take().expect("fail to get stdin");
    let read_stdin = thread::spawn(move || loop {
        // /!\ Manually tested
        let mut buff_writer = BufWriter::new(&mut command_stdin);
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buff_writer.write_all(buffer.as_str().as_bytes()).unwrap();
    });

    let output = Arc::new(Mutex::new(Output::new()));

    let read_stdout = if let Some(stdout) = child.stdout.take() {
        let output = Arc::clone(&output);
        Some(thread::spawn(move || {
            let buf = BufReader::new(stdout);
            let mut buffer = String::new();
            for line in buf.lines() {
                let line = line.unwrap();
                writeln!(&mut buffer, "{}", line).unwrap();
                println!("{}", line.normal().clear());
            }
            let mut output = output.lock().unwrap();
            output.stdout = buffer;
        }))
    } else {
        None
    };
    let read_err = if let Some(stderr) = child.stderr.take() {
        let output = Arc::clone(&output);
        Some(thread::spawn(move || {
            let buf = BufReader::new(stderr);
            let mut buffer = String::new();
            for line in buf.lines() {
                let line = line.unwrap();
                writeln!(&mut buffer, "{}", line).unwrap();
                println!("{}", line.red());
            }
            let mut output = output.lock().unwrap();
            output.stderr = buffer;
        }))
    } else {
        None
    };

    if let Some(read_err) = read_err {
        read_err.join().expect("fail to wait read_err");
    }
    if let Some(read_stdout) = read_stdout {
        read_stdout.join().expect("fail to wait read_stdout");
    }
    drop(read_stdin);

    let exit_status = child.wait().unwrap();
    {
        let mut output = output.lock().unwrap();
        output.status = exit_status.code().unwrap_or_default();
    }

    let output = Arc::try_unwrap(output).unwrap();
    let output = output.into_inner().unwrap();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use cli_integration_test::IntegrationTestEnvironment;

    use crate::run_file::run_as_stream;

    #[test]
    fn run_integration_test_stream() {
        let mut e = IntegrationTestEnvironment::new("run_integration_test");
        e.add_file(
            "run.sh",
            r#"#!/bin/bash
echo TEST
echo ERR >> /dev/stderr
"#,
        );
        e.setup();
        e.set_exec_permission("run.sh").unwrap();

        let output = run_as_stream(
            &e.path().unwrap().join(PathBuf::from("run.sh")),
            &vec![],
            &vec![],
        )
        .unwrap();
        assert_eq!(output.stdout, "TEST\n".to_string());
        assert_eq!(output.stderr, "ERR\n".to_string());
        assert_eq!(output.status, 0);
    }

    #[test]
    fn run_integration_test_stream_with_args() {
        let mut e = IntegrationTestEnvironment::new("run_integration_test");
        e.add_file(
            "run.sh",
            r#"#!/bin/bash
echo ARG = $1
"#,
        );
        e.setup();
        e.set_exec_permission("run.sh").unwrap();

        let output = run_as_stream(
            &e.path().unwrap().join(PathBuf::from("run.sh")),
            &vec![],
            &vec!["TEST_ARG".to_string()],
        )
        .unwrap();
        assert_eq!(output.stdout, "ARG = TEST_ARG\n".to_string());
        assert_eq!(output.status, 0);
    }
}
