use super::Generate;
use crate::cfg::{ArrayVar, ArrayVars, LocalSetupCfg, Vars};
use anyhow::Result;
use std::fmt::Write as FmtWrite;
use std::ops::Deref;

type SheBang = String;

pub const SHEBANG_SH: &'static str = "#!/bin/sh";
#[derive(Debug)]
pub struct ShScript(SheBang);
impl Default for ShScript {
    fn default() -> Self {
        Self {
            0: SHEBANG_SH.into(),
        }
    }
}
impl Generate for ShScript {
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
                "declare -r {var}=${env_var}",
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
        let all = ArrayVar::new("all".into(), ".*".into());
        array_vars.add(all);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::cfg::{ArrayVar, ArrayVars, Vars};
    use crate::run_file::kind::{Generate, ShScript};

    #[test]
    fn generate() {
        let script = ShScript::default();

        let mut array_vars = ArrayVars::new();
        array_vars.add(ArrayVar::new("all".into(), ".*".into()));

        let mut vars = Vars::new();
        vars.add("SETUP_NAME".into());

        let content = script.generate(&array_vars, &vars).unwrap();

        assert_eq!(
            r#"#!/bin/sh
declare -r all=$ALL
declare -r setup_name=$SETUP_NAME

declare -p all
declare -p setup_name
"#,
            content
        );
    }
}
