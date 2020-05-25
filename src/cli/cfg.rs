use crate::cfg::Cfg;
use anyhow::{Context, Result};
use dirs::home_dir;
use std::env::current_dir;

pub fn get_cfg() -> Result<Cfg> {
    let local_dir = current_dir().context("fail to found current directory")?;
    let global_dir = home_dir().context("fail to found home directory")?;

    Cfg::load_local(global_dir, local_dir).context("fail to load cfg")
}
