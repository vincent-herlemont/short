use crate::cfg::Cfg;
use crate::cli::cfg::get_cfg;
use crate::cli::error::CliError;
use crate::cli::settings::{get_settings, Settings};
use crate::cli::terminal::message::success;
use anyhow::{Result};
use clap::ArgMatches;
use std::path::PathBuf;

pub fn env_dir(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app);

    if let Some(env_dir) = app.value_of("env_dir") {
        set(cfg, settings, env_dir.into())
    } else if app.is_present("unset") {
        unset(cfg, settings)
    } else {
        unreachable!()
    }
}

fn set(cfg: Cfg, settings: Settings, env_dir: PathBuf) -> Result<()> {
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

fn unset(cfg: Cfg, settings: Settings) -> Result<()> {
    let setup_name = settings.setup()?;
    let setup = cfg.current_setup(setup_name)?;

    let local_setup = setup.local_setup().unwrap();
    let mut local_setup = local_setup.borrow_mut();
    local_setup.unset_public_env_dir()?;
    drop(local_setup);

    cfg.save()?;

    success(format!("env directory unset").as_str());

    Ok(())
}
