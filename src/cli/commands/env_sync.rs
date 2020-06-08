use crate::cli::cfg::get_cfg;

use crate::cli::settings::get_settings;
use crate::cli::terminal::confirm::{confirm, EnumConfirm};
use crate::cli::terminal::message::success;
use crate::env_file::{Env, EnvDiffController};
use anyhow::{Context, Result};
use clap::ArgMatches;
use filetime::FileTime;
use std::borrow::Cow;

use crate::cli::error::CliError;
use std::fs;
use std::rc::Rc;

enum_confirm!(SyncConfirmEnum, y, n);

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
    let action_delete = app.is_present("delete");
    let action_no_delete = app.is_present("no_delete");

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
        let env_name = Rc::new(env.name()?);
        let env_name_update_var = Rc::clone(&env_name);
        let env_name_delete_var = Rc::clone(&env_name);

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
                            "Updating var in `{}`, `{}`=`{}`. Change value ?",
                            env_name_update_var,
                            var.name(),
                            var.value()
                        )
                        .as_str(),
                        SyncConfirmEnum::to_vec(),
                    ) {
                        break r;
                    }
                };

                let new_value = match &r {
                    SyncConfirmEnum::y => {
                        println!("New value `{}`=", var.name());
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
            move |var| {
                if action_delete {
                    return Ok(true);
                }
                if action_no_delete {
                    return Err(CliError::DeleteVarNowAllowed(
                        var.name().clone(),
                        var.value().clone(),
                        env_name_delete_var.to_string(),
                    )
                    .into());
                }

                let r = loop {
                    let stdin = std::io::stdin();
                    let input = stdin.lock();
                    let output = std::io::stdout();
                    if let Ok(r) = confirm(
                        input,
                        output,
                        format!(
                            "Deleting var in `{}`, `{}`=`{}` ?",
                            env_name_delete_var,
                            var.name(),
                            var.value()
                        )
                        .as_str(),
                        SyncConfirmEnum::to_vec(),
                    ) {
                        break r;
                    }
                };
                if let SyncConfirmEnum::y = r {
                    Ok(true)
                } else {
                    Err(CliError::EnvFileMustBeSync.into())
                }
            },
        );

        env.update_by_diff(&recent_env, &controller)
            .context((CliError::EnvFileMustBeSync).to_string())?;
        env.save().unwrap();
    }

    success("files synchronized");

    Ok(())
}
