use std::env::current_dir;
use std::path::PathBuf;

use anyhow::{Context, Result};
use dirs::home_dir;

use crate::cfg::Cfg;

type LocalDir = PathBuf;
type GlobalDir = PathBuf;

pub fn reach_directories() -> Result<(LocalDir, GlobalDir)> {
    let local_dir = current_dir().context("fail to found current directory")?;
    let global_dir = home_dir().context("fail to found home directory")?;

    Ok((local_dir, global_dir))
}

pub fn get_cfg() -> Result<Cfg> {
    let (local_dir, global_dir) = reach_directories()?;

    Cfg::load_local(global_dir, local_dir).context("fail to load cfg")
}

pub fn create_cfg() -> Result<Cfg> {
    let (local_dir, global_dir) = reach_directories()?;

    Cfg::create_local(global_dir, local_dir).context("fail to create cfg")
}
