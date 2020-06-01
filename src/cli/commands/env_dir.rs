use crate::cli::cfg::get_cfg;
use crate::cli::error::CliError;
use crate::cli::settings::get_settings;
use crate::cli::terminal::message::success;
use anyhow::{Context, Result};
use clap::ArgMatches;
use std::path::PathBuf;

pub fn env_dir(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let env_dir: PathBuf = app.value_of("env_dir").unwrap().into();

    let settings = get_settings(app);
    let setup_name = settings.setup()?;
    let setup = cfg.current_setup(setup_name)?;

    let local_setup = setup.local_setup().unwrap();
    let mut local_setup = local_setup.borrow_mut();
    local_setup.set_public_env_dir(env_dir.clone());
    drop(local_setup);

    let public_env_dir = setup.envs_public_dir()?;
    public_env_dir
        .canonicalize()
        .map_err(|err| CliError::EnvDirNotFound(env_dir.clone(), setup_name.clone(), err))?;

    cfg.save()?;

    success(format!("env directory set to `{:?}`", public_env_dir).as_str());

    Ok(())
}
