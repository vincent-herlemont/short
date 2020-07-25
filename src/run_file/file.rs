use super::kind::{Generate, Kind};
use anyhow::Result;
use fs_extra::file::write_all;
use std::fmt::Write as FmtWrite;
use std::fs::{create_dir_all, set_permissions, Permissions};
use std::ops::Deref;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::str::FromStr;

use crate::cfg::{ArrayVars, Vars};

pub struct File {
    path: PathBuf,
    kind: Kind,
    content: String,
}

impl File {
    pub fn new<S: AsRef<str>>(path: PathBuf, kind: S) -> Result<Self> {
        let kind = Kind::from_str(kind.as_ref())?;
        Ok(Self {
            path,
            kind,
            content: String::new(),
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn generate<AV, V>(&mut self, array_vars: AV, vars: V) -> Result<()>
    where
        AV: Deref<Target = ArrayVars>,
        V: Deref<Target = Vars>,
    {
        self.content = self.kind.generate(array_vars, vars)?;
        Ok(())
    }

    pub fn append(&mut self, code: &str) -> Result<()> {
        writeln!(&mut self.content, "")?;
        writeln!(&mut self.content, "{}", code)?;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        if self.path.exists() {
            bail!("file `{:?}` already exists", self.path);
        }
        if let Some(parent) = self.path.parent() {
            create_dir_all(parent)?;
        }
        write_all(&self.path, self.content.as_str())?;
        set_exec_permision(&self.path)?;
        Ok(())
    }
}

pub fn set_exec_permision(file: &PathBuf) -> Result<()> {
    let file = file.canonicalize()?;
    let permissions = Permissions::from_mode(0o755);
    set_permissions(file, permissions)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use cli_integration_test::IntegrationTestEnvironment;

    use crate::cfg::{ArrayVar, ArrayVars, Vars};
    use crate::run_file::file::{File, Kind};
    use crate::run_file::kind::ShScript;

    #[test]
    fn file_append() {
        let mut file = File {
            path: "".into(),
            content: "".into(),
            kind: Kind::Sh(ShScript::default()),
        };
        file.append("code_1").unwrap();
        file.append("code_2").unwrap();
        assert_eq!(
            r#"
code_1

code_2
"#,
            file.content
        );
    }

    #[test]
    fn file_new() {
        let mut array_vars = ArrayVars::new();
        array_vars.add(ArrayVar::new("all".into(), ".*".into()));

        let mut vars = Vars::new();
        vars.add("SETUP_NAME".into());

        let e = IntegrationTestEnvironment::new("file_new");
        e.setup();
        let path_file = e.path().unwrap().join("run.sh");

        let mut file = File::new(path_file.clone(), "bash").unwrap();
        file.generate(&array_vars, &vars).unwrap();
        file.save().unwrap();
        assert!(path_file.exists());
        let file = e.read_file("./run.sh");
        assert_eq!(
            r#"#!/bin/bash
declare -A all && eval all=($ALL)
declare -r setup_name=$SETUP_NAME

declare -p all
declare -p setup_name
"#,
            file
        );
    }
}
