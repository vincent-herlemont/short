use crate::cli::cfg::get_cfg;
use crate::cli::terminal::message::success;

use anyhow::Result;
use clap::ArgMatches;

use crate::cfg::Cfg;
use crate::cli::settings::{get_settings};
use crate::env_file::{path_from_env_name, Env};

pub fn env_new(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app);

    let setup_name = settings.setup()?;
    let env_name: String = app.value_of("name").unwrap().into();
    let private = app.is_present("private");

    let env = env_new_workflow(&cfg, &setup_name, &env_name, &private)?;

    success(format!("env `{}` created : `{:?}`", env_name, env.file()).as_str());
    Ok(())
}

pub fn env_new_workflow(
    cfg: &Cfg,
    setup_name: &String,
    env_name: &String,
    private: &bool,
) -> Result<Env> {
    let setup = cfg.current_setup(setup_name)?;

    let env_dir = if *private {
        setup.envs_private_dir()?
    } else {
        setup.envs_public_dir()?
    };

    let env = path_from_env_name(env_dir, &env_name);
    let env: Env = env.into();
    env.save()?;
    Ok(env)
}
