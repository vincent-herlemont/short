use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{Context, Result};
use clap::ArgMatches;
use log::*;

use crate::cli::cfg::get_cfg;
use crate::cli::error::CliError;
use crate::cli::settings::get_settings;
use crate::cli::terminal::confirm::{confirm, EnumConfirm};
use crate::cli::terminal::message::success;
use crate::env_file::{Env, EnvDiffController};

#[derive(Debug)]
pub struct SyncSettings {
    pub empty: bool,
    pub copy: bool,
    pub delete: bool,
    pub no_delete: bool,
    pub file: Option<String>,
}

impl SyncSettings {
    pub fn new(args: &ArgMatches) -> Self {
        Self {
            empty: args.is_present("empty"),
            copy: args.is_present("copy"),
            delete: args.is_present("delete"),
            no_delete: args.is_present("no_delete"),
            file: args.value_of("file").map(|f| f.to_string()),
        }
    }
}

enum_confirm!(SyncConfirmEnum, y, n);

pub fn env_sync(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app, &cfg);
    let sync_settings = SyncSettings::new(app);

    let setup = cfg.current_setup(settings.setup()?)?;
    let envs = setup.envs();
    let envs: Vec<_> = envs.into_iter().filter_map(|r| r.ok()).collect();

    let recent_env = Env::recent(&envs)?;

    sync_workflow(recent_env, envs, sync_settings)?;

    success("files synchronized");

    Ok(())
}

pub fn sync_workflow(
    source_env: Env,
    envs: Vec<Env>,
    sync_settings: SyncSettings,
) -> Result<Vec<Env>> {
    let source_env = if let Some(file) = sync_settings.file.as_ref() {
        Env::from_file_reader(file)?
    } else {
        source_env
    };
    let envs: Vec<_> = envs.into_iter().map(|env| RefCell::new(env)).collect();

    let sync_settings = Rc::new(sync_settings);

    for env_cell in envs.iter() {
        let mut env = env_cell.borrow_mut();
        if env.file() == source_env.file() {
            continue;
        }
        let env_name = Rc::new(env.name()?);
        let env_name_update_var = Rc::clone(&env_name);
        let env_name_delete_var = Rc::clone(&env_name);

        let sync_settings_update_var = Rc::clone(&sync_settings);
        let sync_settings_delete_var = Rc::clone(&sync_settings);

        let controller = EnvDiffController::new(
            move |var| {
                info!("{:?}", &sync_settings_update_var);
                if sync_settings_update_var.empty {
                    var.set_value("");
                    return Cow::Borrowed(var);
                }
                if sync_settings_update_var.copy {
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
                if sync_settings_delete_var.delete {
                    return Ok(true);
                }
                if sync_settings_delete_var.no_delete {
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
                    Err(CliError::DeleteVarNowAllowed(
                        var.name().clone(),
                        var.value().clone(),
                        env_name_delete_var.to_string(),
                    )
                    .into())
                }
            },
        );
        env.update_by_diff(&source_env, &controller)
            .context((CliError::EnvFileMustBeSync).to_string())?;
        env.save().unwrap();
    }

    let envs: Vec<_> = envs.into_iter().map(|env| env.into_inner()).collect();
    Ok(envs)
}
