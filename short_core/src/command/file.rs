use crate::cfg::EnvGroups;
use crate::command::{generate_vars, Var};
use anyhow::{Context, Result};
use fs_extra::file::write_all;
use short_env::Env;
use std::fmt::Write as FmtWrite;
use std::fs::{set_permissions, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub struct File {
    path: PathBuf,
    shebang: String,
}

impl File {
    pub fn new(path: PathBuf, shebang: String) -> Self {
        Self { path, shebang }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn shebang(&self) -> &String {
        &self.shebang
    }
}

pub fn set_exec_permision(file: &PathBuf) -> Result<()> {
    let file = file.canonicalize()?;
    let permissions = Permissions::from_mode(0o755);
    set_permissions(file, permissions)?;
    Ok(())
}

pub fn new(file: &File, vars: &Vec<Var>) -> Result<()> {
    let mut content = String::new();
    writeln!(&mut content, "{}", file.shebang);

    if vars.len() > 0 {
        for var in vars.iter() {
            if var.array {
                writeln!(
                    &mut content,
                    "declare -A {name} && eval {name}=(${env_name})",
                    name = var.name,
                    env_name = var.env_name
                );
            } else {
                writeln!(&mut content, "declare -r {}=${}", var.name, var.env_name);
            }
        }
        writeln!(&mut content, "");
        for var in vars.iter() {
            writeln!(&mut content, "declare -p {}", var.name);
        }
    }

    write_all(&file.path, content.as_str())?;
    set_exec_permision(&file.path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cfg::EnvGroups;
    use crate::command::env_group::Var;
    use crate::command::file::{new, File};
    use predicates::prelude::predicate::path::exists;
    use predicates::prelude::*;
    use short_env::Env;
    use short_utils::integration_test::environment::IntegrationTestEnvironment;

    #[test]
    fn file_new() {
        let vars = vec![
            Var {
                array: true,
                name: "all".into(),
                env_name: "ALL".into(),
                env_value: " [VAR1]=\'VALUE1\' [VAR2]=\'VALUE2\' ".into(),
            },
            Var {
                array: false,
                name: "var1".into(),
                env_name: "VAR1".into(),
                env_value: "VALUE1".into(),
            },
        ];

        let mut e = IntegrationTestEnvironment::new("command");
        e.setup();
        let file = e.path().join("run.sh");
        new(&File::new(file.clone(), String::from("#!/bin/bash")), &vars).unwrap();
        assert!(file.exists());
        let file = e.read_file("./run.sh");
        assert_eq!(
            r#"#!/bin/bash
declare -A all && eval all=($ALL)
declare -r var1=$VAR1

declare -p all
declare -p var1
"#,
            file
        );
    }
}
