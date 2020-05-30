mod file;
mod var;

pub use file::{set_exec_permision, File};
pub use var::{generate_array_env_var, generate_env_var, generate_env_vars, EnvVar};

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process;
use std::process::Command;

#[derive(Debug)]
pub struct Output {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
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

pub fn run(file: &PathBuf, vars: &Vec<EnvVar>) -> Result<Output> {
    let file = file.canonicalize()?;
    let mut output = Command::new(&file);
    output.env_clear();

    for env_var in vars.iter() {
        output.env(env_var.var().to_env_var(), env_var.env_value());
    }

    let output = output
        .output()
        .context(format!("command {} fail", file.to_string_lossy()))?;

    Ok(output.into())
}

#[cfg(test)]
mod tests {
    use crate::cfg::{ArrayVars, Vars};
    use crate::env_file::Env;
    use crate::run_file::file::File;
    use crate::run_file::run;
    use crate::run_file::var::generate_env_vars;
    use cli_integration_test::IntegrationTestEnvironment;

    #[test]
    fn run_integration_test() {
        let e = IntegrationTestEnvironment::new("run_integration_test");
        e.setup();

        let mut array_vars = ArrayVars::new();
        array_vars.add("all".into(), ".*".into());

        let mut vars = Vars::new();
        vars.add("SETUP_NAME".into());

        let mut env = Env::new();
        env.add("VAR1", "VALUE1");
        env.add("SETUP_NAME", "VALUE2");

        let path_file = e.path().join("run.sh");
        let mut file = File::new(path_file.clone(), String::from("#!/bin/bash"));
        file.generate(&array_vars, &vars).unwrap();
        file.save().unwrap();

        let env_vars = generate_env_vars(&env, &array_vars, &vars).unwrap();

        let output = run(&path_file, &env_vars).unwrap();
        assert_eq!(
            &output.stdout,
            r#"declare -A all=([VAR1]="VALUE1" [SETUP_NAME]="VALUE2" )
declare -r setup_name="VALUE2"
"#
        );
    }
}
