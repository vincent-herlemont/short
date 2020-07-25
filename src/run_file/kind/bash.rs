use super::Generate;
use crate::cfg::{ArrayVar, ArrayVars, LocalSetupCfg, VarCase, Vars};
use anyhow::Result;
use std::fmt::Write as FmtWrite;
use std::ops::Deref;

type SheBang = String;

pub const SHEBANG_BASH: &'static str = "#!/bin/bash";

#[derive(Debug)]
pub struct BashScript(SheBang);
impl Default for BashScript {
    fn default() -> Self {
        Self {
            0: SHEBANG_BASH.into(),
        }
    }
}
impl Generate for BashScript {
    fn generate<AV, V>(&self, array_vars: AV, vars: V) -> Result<String>
    where
        AV: Deref<Target = ArrayVars>,
        V: Deref<Target = Vars>,
    {
        let mut content = String::new();
        writeln!(content, "{}", self.0)?;

        let mut defined_vars = vec![];

        for array_var in array_vars.as_ref() {
            let var = array_var.var();
            writeln!(
                content,
                "declare -A {var} && eval {var}=(${env_var})",
                var = var.to_var(),
                env_var = var.to_env_var()
            )?;
            defined_vars.append(&mut vec![var])
        }

        for var in vars.as_ref() {
            writeln!(
                content,
                "declare -r {var}=${env_var}",
                var = var.to_var(),
                env_var = var.to_env_var(),
            )?;
            defined_vars.append(&mut vec![var])
        }

        writeln!(content, "")?;
        for var in defined_vars.iter() {
            writeln!(content, "declare -p {}", var.to_var())?;
        }
        Ok(content)
    }

    fn update_local_setup_cfg(&self, local_setup_cfg: &mut LocalSetupCfg) -> Result<()> {
        let array_vars = local_setup_cfg.new_array_vars();
        let mut array_vars = array_vars.borrow_mut();
        let mut all = ArrayVar::new("all".into(), ".*".into());
        all.set_delimiter(" ".into());
        all.set_format("[{key}]='{value}'".into());
        all.set_case(VarCase::CamelCase);
        array_vars.add(all);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::cfg::{ArrayVar, ArrayVars, Vars};
    use crate::run_file::kind::{BashScript, Generate};

    #[test]
    fn generate() {
        let script = BashScript::default();

        let mut array_vars = ArrayVars::new();
        array_vars.add(ArrayVar::new("all".into(), ".*".into()));

        let mut vars = Vars::new();
        vars.add("SETUP_NAME".into());

        let content = script.generate(&array_vars, &vars).unwrap();

        assert_eq!(
            r#"#!/bin/bash
declare -A all && eval all=($ALL)
declare -r setup_name=$SETUP_NAME

declare -p all
declare -p setup_name
"#,
            content
        )
    }
}
