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
    let action_empty = app.is_present("empty");
    let action_not_change = app.is_present("no_change");

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
                if action_empty {
                    var.set_value("");
                    return Cow::Borrowed(var);
                }
                if action_not_change {
                    return Cow::Borrowed(var);
                }

                let r = loop {
                    let stdin = std::io::stdin();
                    let input = stdin.lock();
                    let output = std::io::stdout();
                    if let Ok(r) = confirm(
                        input,
                        output,
                        format!(
                            "Updating `{}`, do you want to change value of `{}`=`{}` ?",
                            env_name,
                            var.name(),
                            var.value()
                        )
                        .as_str(),
                        UpdateEnum::to_vec(),
                    ) {
                        break r;
                    }
                };

                let new_value = match &r {
                    UpdateEnum::y => {
                        println!("new value `{}`=", var.name());
                        let mut new_value = String::new();
                        std::io::stdin().read_line(&mut new_value).unwrap();
                        Some(new_value)
                    }
                    _ => None,
                };

                if let Some(new_value) = new_value {
                    var.set_value(new_value.as_str());
                    Cow::Borrowed(var)
                } else {
                    Cow::Borrowed(var)
                }
            },
            |_| true,
        );

        env.update_by_diff(&recent_env, &controller);
        env.save().unwrap();
    }

    success("files synchronized");

    Ok(())
}
