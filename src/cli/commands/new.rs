use anyhow::Result;
use clap::ArgMatches;
use colored::*;
use std::path::PathBuf;

use crate::cfg::Cfg;
use crate::cli::cfg::get_cfg;
use crate::cli::commands::sync::{sync_workflow, SyncSettings};
use crate::cli::error::CliError;
use crate::cli::settings::get_settings;
use crate::cli::terminal::message::success;
use crate::env_file::{path_from_env_name, Env};

use super::r#use::use_workflow;

pub fn env_new(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let mut settings = get_settings(app, &cfg);
    let sync_settings = SyncSettings::new(&app);

    let setup_name = settings.setup()?;
    let env_name: String = app.value_of("name").unwrap().into();
    let private = app.is_present("private");

    let setup = cfg.current_setup(setup_name)?;
    let mut envs = setup.envs().into_iter().filter_map(|r| r.ok()).collect();
    let recent_env = Env::recent(&envs);

    let new_env = env_new_workflow(&cfg, &setup_name, &env_name, &private, &false)?;
    envs.push(new_env.clone());

    if let Ok(recent_env) = recent_env {
        if let Err(e) = sync_workflow(recent_env.clone(), envs, sync_settings) {
            // Remove env file when sync is stopped/fail.
            new_env.remove()?;
            return Err(e);
        }
    }

    settings.set_env(new_env.name()?);
    use_workflow(&cfg, &settings)?;
    cfg.save()?;

    success(format!("env `{}` created : `{:?}`", env_name.bold(), new_env.file()).as_str());
    Ok(())
}

pub fn env_new_workflow(
    cfg: &Cfg,
    setup_name: &String,
    env_name: &String,
    private: &bool,
    example: &bool,
) -> Result<Env> {
    let setup = cfg.current_setup(setup_name)?;

    let retrieve_env_is_not_exists = |dir: PathBuf| -> Result<Env> {
        let env = path_from_env_name(dir, env_name);
        let mut env: Env = env.into();
        if *example {
            env.add("VAR1", "VALUE1");
            env.add("VAR2", "VALUE2");
        }
        if env.file().exists() {
            return Err(CliError::EnvFileAlreadyExists(env.file().clone(), env.clone()).into());
        } else {
            Ok(env)
        }
    };

    let public_env = setup.envs_public_dir().map(retrieve_env_is_not_exists);
    if let Ok(Err(err)) = public_env {
        return Err(err);
    };

    let private_env = setup.envs_private_dir().map(retrieve_env_is_not_exists);
    if let Ok(Err(err)) = private_env {
        return Err(err);
    };

    let env = if *private {
        private_env??
    } else {
        public_env??
    };
    env.save()?;

    Ok(env)
}
