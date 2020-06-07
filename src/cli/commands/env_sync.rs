use crate::cli::cfg::get_cfg;

use crate::cli::settings::get_settings;
use crate::cli::terminal::confirm::{confirm, EnumConfirm};
use crate::cli::terminal::message::success;
use crate::env_file::{Env, EnvDiffController};
use anyhow::{Context, Result};
use clap::ArgMatches;
use filetime::FileTime;
use std::borrow::Cow;

use std::fs;


enum_confirm!(UpdateEnum, y, n);

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
    let envs: Vec<_> = envs.into_iter().filter_map(|r| r.ok()).collect();

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

    for mut env in envs {
        if env.file() == recent_env.file() {
            continue;
        }
        let env_name = env.name()?;

        let controller = EnvDiffController::new(
            move |var| {
                let stdin = std::io::stdin();
                let input = stdin.lock();
                let output = std::io::stdout();
                confirm(
                    input,
                    output,
                    format!(
                        "[{}] Do you want update value of `{}`=`{}` ?",
                        env_name,
                        var.name(),
                        var.value()
                    )
                    .as_str(),
                    UpdateEnum::to_vec(),
                )
                .unwrap();
                Cow::Borrowed(var)
            },
            |_| true,
        );

        env.update_by_diff(&recent_env, &controller);
        env.save().unwrap();
    }

    success("files synchronized");

    Ok(())
}
