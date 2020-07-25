use crate::cfg::{ArrayVars, LocalSetupCfg, Vars};
use anyhow::Result;
use std::ops::Deref;

use strum_macros::AsRefStr;
use strum_macros::EnumCount;
use strum_macros::EnumIter;
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

    fn update_local_setup_cfg(&self, local_setup_cfg: &mut LocalSetupCfg) -> Result<()>;
}

#[derive(EnumString, AsRefStr, EnumIter, EnumCount, Debug)]
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

    fn update_local_setup_cfg(&self, local_setup_cfg: &mut LocalSetupCfg) -> Result<()> {
        match self {
            Kind::Bash(bash) => bash.update_local_setup_cfg(local_setup_cfg),
            Kind::Sh(sh) => sh.update_local_setup_cfg(local_setup_cfg),
        }
    }
}
