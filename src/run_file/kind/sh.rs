use super::Generate;
use crate::cfg::{ArrayVars, Vars};
use anyhow::{Result};
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
    fn generate<AV, V>(&self, _array_vars: AV, _vars: V) -> Result<String>
    where
        AV: Deref<Target = ArrayVars>,
        V: Deref<Target = Vars>,
    {
        Ok(String::from("#generated_sh_script"))
    }
}
