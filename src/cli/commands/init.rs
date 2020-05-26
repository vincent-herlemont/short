use crate::cli::cfg::create_cfg;
use crate::cli::terminal::message::{success};
use anyhow::{Result};
use clap::ArgMatches;

pub fn init(_app: &ArgMatches) -> Result<()> {
    let cfg = create_cfg()?;
    cfg.save()?;
    success("project initialed");
    Ok(())
}
