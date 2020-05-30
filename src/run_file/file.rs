use crate::cfg::{ArrayVar, ArrayVars, Var, Vars};

use anyhow::Result;
use fs_extra::file::write_all;

use std::fmt::Write as FmtWrite;
use std::fs::{set_permissions, Permissions};
use std::ops::Deref;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

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

        for array_var in array_vars.inner() {
            let var_name = array_var.var_name();
            writeln!(
                &mut self.content,
                "declare -A {var} && eval {var}=(${env_var})",
                var = var_name.to_var(),
                env_var = var_name.to_env_var()
            )?;
            defined_vars.append(&mut vec![var_name])
        }

        for var_name in vars.inner() {
            writeln!(
                &mut self.content,
                "declare -r {var}=${var_name}",
                var = var_name.to_var(),
                var_name = var_name.to_env_var(),
            )?;
            defined_vars.append(&mut vec![var_name])
        }

        writeln!(&mut self.content, "")?;
        for var_name in defined_vars.iter() {
            writeln!(&mut self.content, "declare -p {}", var_name.to_var())?;
        }

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
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

    use crate::cfg::{ArrayVars, Vars};
    use crate::run_file::file::File;

    use cli_integration_test::IntegrationTestEnvironment;

    #[test]
    fn file_new() {
        let mut array_vars = ArrayVars::new();
        array_vars.add("all".into(), ".*".into());

        let mut vars = Vars::new();
        vars.add("SETUP_NAME".into());

        let e = IntegrationTestEnvironment::new("command");
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
