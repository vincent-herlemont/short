use crate::cli::cfg::get_cfg;

use crate::cli::terminal::message::success;
use anyhow::{Context, Result};
use clap::ArgMatches;

pub fn rename(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;

    let last_setup_name = app.value_of("last_setup_name").unwrap();
    let new_setup_name = app.value_of("new_setup_name").unwrap();

    let setup = cfg.current_setup(&last_setup_name.to_string())?;

    setup.rename(&new_setup_name.to_string())?;

    cfg.save()?;

    success("setup renamed");

    Ok(())
}
