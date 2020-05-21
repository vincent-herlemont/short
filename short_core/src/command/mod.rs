mod file;

pub use file::new;
pub use file::set_exec_permision;

use anyhow::{Context, Result};
use std::fs::{metadata, set_permissions, Permissions};
use std::path::PathBuf;
use std::process::Command;

fn run(file: &PathBuf) -> Result<()> {
    let file = file.canonicalize()?;
    let output = Command::new(&file)
        .env_clear()
        .env("ALL", " [k1]=a [k2]=b ")
        .output()
        .context(format!("command {} fail", file.to_string_lossy()))?;
    dbg!(&output);
    let stdout = String::from_utf8(output.stdout)?;
    println!("{}", stdout);
    let stderr = String::from_utf8(output.stderr)?;
    println!("{}", stderr);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::command::{run, set_exec_permision};
    use predicates::prelude::*;
    use short_utils::integration_test::environment::IntegrationTestEnvironment;

    #[test]
    fn simple() {
        let mut e = IntegrationTestEnvironment::new("command");
        e.add_file(
            "run.sh",
            r#"#!/bin/sh
declare -A all && eval all=($ALL)
declare -p all
"#,
        );
        e.setup();
        let file = e.path().join("run.sh");
        set_exec_permision(&file).unwrap();
        run(&file).unwrap();
    }
}
