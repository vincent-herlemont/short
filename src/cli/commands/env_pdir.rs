use crate::cli::cfg::get_cfg;
use crate::cli::error::CliError;
use crate::cli::settings::get_settings;
use crate::cli::terminal::message::success;
use anyhow::{Context, Result};
use clap::ArgMatches;
use std::env::current_dir;
use std::path::PathBuf;

pub fn env_pdir(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let env_dir: PathBuf = app.value_of("env_dir").unwrap().into();

    let settings = get_settings(app);
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
