use crate::cli::cfg::get_cfg;
use crate::cli::error::CliError;
use crate::cli::settings::get_settings;
use crate::cli::terminal::message::success;
use anyhow::{Context, Result};
use clap::ArgMatches;
use std::env;
use std::fs;

use crate::env_file::{Env, EnvDiffController};
use filetime::FileTime;
use std::borrow::Cow;
use std::process::Command;

fn last_modification_time(env: &Env) -> FileTime {
    let file = env.file();
    let metadata = fs::metadata(file).unwrap();
    FileTime::from_last_modification_time(&metadata)
}

pub fn env_sync(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app, &cfg);

    let setup = cfg.current_setup(settings.setup()?)?;
    let envs = setup.envs();
    let mut envs: Vec<_> = envs.into_iter().filter_map(|r| r.ok()).collect();

    let recent_env = envs
        .iter()
        .fold(None, |last_env, next_env| match (last_env, next_env) {
            (None, next_env) => Some(next_env.clone()),
            (Some(last_env), next_env) => {
                let last_env_filetime = last_modification_time(&last_env);
                let next_env_filetime = last_modification_time(next_env);
                if last_env_filetime < next_env_filetime {
                    Some((*next_env).clone())
                } else {
                    Some(last_env)
                }
            }
        });
    let recent_env = recent_env.context("fail to found the most recent env file")?;

    let controller = EnvDiffController::new(|var| Cow::Borrowed(var), |_| true);

    for mut env in envs {
        if env.file() == recent_env.file() {
            continue;
        }

        env.update_by_diff(&recent_env, &controller);
        env.save().unwrap();
    }

    success("files synchronized");

    Ok(())
}
