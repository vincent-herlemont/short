use super::Generate;
use crate::cfg::{ArrayVars, Vars};
use anyhow::{Result};
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
}
