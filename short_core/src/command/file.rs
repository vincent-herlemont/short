use anyhow::{Context, Result};
use fs_extra::file::write_all;
use std::fs::{set_permissions, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub fn set_exec_permision(file: &PathBuf) -> Result<()> {
    let file = file.canonicalize()?;
    let permissions = Permissions::from_mode(0o755);
    set_permissions(file, permissions)?;
    Ok(())
}

pub fn new(file: &PathBuf) -> Result<()> {
    let content = r#"#!/bin/sh
declare -A all && eval all=($ALL)
declare -p all
"#;
    write_all(file, content)?;
    set_exec_permision(file)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::command::file::new;
    use predicates::prelude::predicate::path::exists;
    use predicates::prelude::*;
    use short_utils::integration_test::environment::IntegrationTestEnvironment;

    #[test]
    fn simple() {
        let mut e = IntegrationTestEnvironment::new("command");
        e.setup();
        let file = e.path().join("run.sh");
        new(&file).unwrap();
        assert!(file.exists());
    }
}
