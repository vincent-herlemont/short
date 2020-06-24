use std::fmt::Write as FmtWrite;
use std::fs::{create_dir_all, Permissions, set_permissions};
use std::ops::Deref;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use anyhow::Result;
use fs_extra::file::write_all;

use crate::cfg::{ArrayVars, Vars};

pub struct File {
    path: PathBuf,
    shebang: String,
    content: String,
}

impl File {
    pub fn new(path: PathBuf, shebang: String) -> Self {
        Self {
            path,
            shebang,
            content: String::new(),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn shebang(&self) -> &String {
        &self.shebang
    }

    pub fn generate<AV, V>(&mut self, array_vars: AV, vars: V) -> Result<()>
    where
        AV: Deref<Target = ArrayVars>,
        V: Deref<Target = Vars>,
    {
        writeln!(&mut self.content, "{}", self.shebang)?;

        let mut defined_vars = vec![];

        for array_var in array_vars.as_ref() {
            let var = array_var.var();
            writeln!(
                &mut self.content,
                "declare -A {var} && eval {var}=(${env_var})",
                var = var.to_var(),
                env_var = var.to_env_var()
            )?;
            defined_vars.append(&mut vec![var])
        }

        for var in vars.as_ref() {
            writeln!(
                &mut self.content,
                "declare -r {var}=${env_var}",
                var = var.to_var(),
                env_var = var.to_env_var(),
            )?;
            defined_vars.append(&mut vec![var])
        }

        writeln!(&mut self.content, "")?;
        for var in defined_vars.iter() {
            writeln!(&mut self.content, "declare -p {}", var.to_var())?;
        }

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

    use crate::cfg::{ArrayVars, Vars};
    use crate::run_file::file::File;

    #[test]
    fn file_append() {
        let mut file = File {
            path: "".into(),
            content: "".into(),
            shebang: "".into(),
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
        array_vars.add("all".into(), ".*".into());

        let mut vars = Vars::new();
        vars.add("SETUP_NAME".into());

        let e = IntegrationTestEnvironment::new("file_new");
        e.setup();
        let path_file = e.path().join("run.sh");

        let mut file = File::new(path_file.clone(), String::from("#!/bin/bash"));
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
