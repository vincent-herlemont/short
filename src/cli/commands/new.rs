use crate::cfg::LocalSetupCfg;
use crate::cli::cfg::{get_cfg};

use anyhow::{Context, Result};
use clap::ArgMatches;
use std::path::PathBuf;

pub fn new(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    let setup_name = app
        .value_of("setup_name")
        .context("setup name can not have no UTF-8 string")?;

    let setup_file = app
        .value_of("file")
        .context("setup name can not have no UTF-8 string")?;
    let setup_file = PathBuf::from(setup_file);

    let local_setup_cfg = LocalSetupCfg::new(setup_name.into(), setup_file);
    cfg.add_local_setup_cfg(local_setup_cfg);
    cfg.sync_local_to_global()?;
    cfg.save()?;

    Ok(())
}
