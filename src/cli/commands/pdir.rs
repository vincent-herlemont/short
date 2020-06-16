use crate::cfg::Cfg;
use crate::cli::cfg::get_cfg;
use crate::cli::error::CliError;
use crate::cli::settings::{get_settings, Settings};
use crate::cli::terminal::message::success;
use anyhow::Result;
use clap::ArgMatches;
use std::env::current_dir;
use std::path::PathBuf;

pub fn env_pdir(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app, &cfg);

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

    let global_setup = setup.global_setup().unwrap();
    let mut global_setup = global_setup.borrow_mut();
    let private_env_dir = if env_dir.is_relative() {
        current_dir()?.join(&env_dir).canonicalize()
    } else {
        env_dir.canonicalize()
    };
    let private_env_dir = private_env_dir
        .map_err(|err| CliError::EnvDirNotFound(env_dir.clone(), setup_name.clone(), err))?;
    global_setup.set_private_env_dir(private_env_dir.clone())?;
    drop(global_setup);

    cfg.save()?;

    success(format!("private env directory set to `{:?}`", private_env_dir).as_str());

    Ok(())
}

fn unset(cfg: Cfg, settings: Settings) -> Result<()> {
    let setup_name = settings.setup()?;
    let setup = cfg.current_setup(setup_name)?;

    let global_setup = setup.global_setup().unwrap();
    let mut global_setup = global_setup.borrow_mut();
    global_setup.unset_private_env_dir()?;
    drop(global_setup);

    cfg.save();

    success(format!("private env directory unset").as_str());

    Ok(())
}
