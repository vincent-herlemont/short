use crate::cfg::{ArrayVars, Vars};
use anyhow::{Result};
use std::ops::Deref;
use strum_macros::EnumString;

mod bash;
mod sh;

pub use bash::BashScript;
pub use sh::ShScript;

pub trait Generate {
    fn generate<AV, V>(&self, array_vars: AV, vars: V) -> Result<String>
    where
        AV: Deref<Target = ArrayVars>,
        V: Deref<Target = Vars>;
}

#[derive(EnumString, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum Kind {
    #[strum(serialize = "sh", props(deserialize = "sh"))]
    Sh(ShScript),
    #[strum(serialize = "bash", props(deserialize = "bash"))]
    Bash(BashScript),
}

impl Generate for Kind {
    fn generate<AV, V>(&self, array_vars: AV, vars: V) -> Result<String>
    where
        AV: Deref<Target = ArrayVars>,
        V: Deref<Target = Vars>,
    {
        match self {
            Kind::Bash(bash) => bash.generate(array_vars, vars),
            Kind::Sh(sh) => sh.generate(array_vars, vars),
        }
    }
}
