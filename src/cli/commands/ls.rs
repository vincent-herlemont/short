use crate::cli::cfg::get_cfg;
use crate::cli::settings::get_settings;
use crate::cli::terminal::emoji;
use crate::cli::terminal::message::message;
use crate::env_file::Env;
use anyhow::Result;
use clap::ArgMatches;
use log::*;

fn line(msg: &str, r#use: &bool) {
    let c = if *r#use {
        emoji::RIGHT_POINTER.to_string()
    } else {
        " ".to_string()
    };
    message(format!("{} {}", c, msg).as_str());
}

pub fn ls(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app, &cfg);

    let local_setups = cfg.current_setups()?;

    for local_setup in local_setups {
        let setup_name = local_setup.name()?;
        let envs: Vec<Env> = local_setup
            .envs()
            .into_iter()
            .filter_map(|r| {
                if let Err(e) = &r {
                    error!("{}", e);
                }
                r.ok()
            })
            .collect();
        let check = if let (Ok(setting_setup), Err(_)) = (settings.setup(), settings.env()) {
            if setting_setup == &setup_name {
                true
            } else {
                false
            }
        } else {
            false
        };

        line(&setup_name, &check);

        if !envs.is_empty() {
            for env in envs {
                let env_name = match env.name() {
                    Ok(env_name) => env_name,
                    Err(e) => {
                        error!("{}", e);
                        continue;
                    }
                };

                let check = if let (Ok(setting_env), Ok(setting_setup)) =
                    (settings.env(), settings.setup())
                {
                    if setting_env == &env_name && setting_setup == &setup_name {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };

                line(format!("   {}", &env_name).as_str(), &check);
            }
        }
    }
    Ok(())
}
