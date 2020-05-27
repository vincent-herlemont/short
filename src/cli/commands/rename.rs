
use crate::cli::cfg::get_cfg;

use crate::cli::terminal::message::success;
use anyhow::{Context, Result};
use clap::ArgMatches;


pub fn rename(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;

    let last_setup_name = app
        .value_of("last_setup_name")
        .context("last setup name can not have no UTF-8 string")?;
    let new_setup_name = app
        .value_of("new_setup_name")
        .context("new setup name can not have no UTF-8 string")?;

    let setup = cfg
        .local_setup(&last_setup_name.to_string())?
        .context(format!("fail to found setup \"{}\"", last_setup_name))?;

    setup.rename(&new_setup_name.to_string())?;

    cfg.save()?;

    success("setup renamed");

    Ok(())
}
