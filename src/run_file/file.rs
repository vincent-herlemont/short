use crate::run_file::Var;
use anyhow::Result;
use fs_extra::file::write_all;
use std::fmt::Write as FmtWrite;
use std::fs::{set_permissions, Permissions};
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

    pub fn generate(&mut self, vars: &Vec<Var>) -> Result<()> {
        writeln!(&mut self.content, "{}", self.shebang)?;

        if vars.len() > 0 {
            for var in vars.iter() {
                if var.array {
                    writeln!(
                        &mut self.content,
                        "declare -A {name} && eval {name}=(${env_name})",
                        name = var.name,
                        env_name = var.env_name
                    )?;
                } else {
                    writeln!(
                        &mut self.content,
                        "declare -r {}=${}",
                        var.name, var.env_name
                    )?;
                }
            }
            writeln!(&mut self.content, "")?;
            for var in vars.iter() {
                writeln!(&mut self.content, "declare -p {}", var.name)?;
            }
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

    use crate::run_file::array_var::Var;
    use crate::run_file::file::File;
    use cli_integration_test::IntegrationTestEnvironment;

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

        let e = IntegrationTestEnvironment::new("command");
        e.setup();
        let path_file = e.path().join("run.sh");
        let mut file = File::new(path_file.clone(), String::from("#!/bin/bash"));
        file.generate(&vars);
        file.save();
        assert!(path_file.exists());
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
