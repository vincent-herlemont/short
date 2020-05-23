mod env_group;
mod file;

pub use env_group::{generate_var, generate_vars, Var};
pub use file::new;
pub use file::set_exec_permision;

use anyhow::{Context, Result};
use std::fs::{metadata, set_permissions, Permissions};
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

fn run(file: &PathBuf, vars: &Vec<Var>) -> Result<Output> {
    let file = file.canonicalize()?;
    let mut output = Command::new(&file);
    output.env_clear();

    for var in vars.iter() {
        output.env(&var.env_name, &var.env_value);
    }

    let output = output
        .output()
        .context(format!("command {} fail", file.to_string_lossy()))?;

    Ok(output.into())
}

#[cfg(test)]
mod tests {
    use crate::cfg::EnvGroups;
    use crate::command::{generate_vars, new, run, set_exec_permision};
    use predicates::prelude::*;
    use short_env::Env;
    use short_utils::integration_test::environment::IntegrationTestEnvironment;

    #[test]
    fn run_integration_test() {
        let mut e = IntegrationTestEnvironment::new("run_integration_test");
        e.setup();

        let mut env_groups = EnvGroups::new();
        env_groups.add("all".into(), ".*".into());
        env_groups.add("var1".into(), "VAR1".into());
        let mut env_file = Env::new();
        env_file.add("VAR1", "VALUE1");
        env_file.add("VAR2", "VALUE2");
        let vars = generate_vars(&env_file, &env_groups).unwrap();

        let file = e.path().join("run.sh");
        new(&file, &vars).unwrap();
        let output = run(&file, &vars).unwrap();
        assert_eq!(&output.status, &0);
        assert_eq!(
            &output.stdout,
            "declare -A all=([VAR1]=\"VALUE1\" [VAR2]=\"VALUE2\" )\ndeclare -r var1=\"VALUE1\"\n"
        );
    }
}
