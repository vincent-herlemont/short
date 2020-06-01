
use crate::cli::cfg::get_cfg;
use crate::cli::terminal::message::success;

use anyhow::{Result};
use clap::ArgMatches;

use crate::cli::settings::get_settings;
use crate::env_file::{path_from_env_name, Env};


pub fn env_new(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let env_name = app.value_of("name").unwrap();

    let settings = get_settings(app);
    let setup_name = settings.setup()?;
    let setup = cfg.current_setup(setup_name)?;

    let env_dir = if app.is_present("private") {
        setup.envs_private_dir()?
    } else {
        setup.envs_public_dir()?
    };

    let env = path_from_env_name(env_dir, &env_name.to_string());
    let env: Env = env.into();
    env.save()?;

    success(format!("env `{}` created : `{:?}`", env_name, env.file()).as_str());

    Ok(())
}
