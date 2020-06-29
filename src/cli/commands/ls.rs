use anyhow::Result;
use clap::ArgMatches;
use log::*;

use crate::cli::cfg::get_cfg;
use crate::cli::settings::get_settings;
use crate::cli::terminal::emoji;
use crate::cli::terminal::message::message;
use crate::env_file::Env;

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

    let project = cfg.current_project()?;
    let project = project.borrow();
    let local_setups = cfg.current_setups()?;

    for local_setup in local_setups {
        let setup_name = local_setup.name()?;
        let check = if let (Ok(setting_setup), Err(_)) = (settings.setup(), settings.env()) {
            if setting_setup == &setup_name {
                true
            } else {
                false
            }
        } else {
            false
        };

        let local_setup_cfg = local_setup.local_setup().unwrap();
        let local_setup_cfg = local_setup_cfg.borrow();
        let run_file = local_setup_cfg.file();
        line(
            format!("{} ({})", &setup_name, run_file.to_string_lossy()).as_str(),
            &check,
        );

        let envs: Vec<Env> = local_setup
            .envs()
            .into_iter()
            .filter_map(|r| {
                // TODO : add exclude `.<file_name>` that not an env like .gitignore
                //        and display errors
                // if let Err(e) = &r {
                //     error!("{}", e);
                //     e.chain().skip(1).for_each(|cause| error!("{}", cause));
                // }
                r.ok()
            })
            .collect();
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

                let env_file = env
                    .file()
                    .strip_prefix(project.dir()?)
                    .unwrap_or(env.file());

                line(
                    format!("   {} ({})", &env_name, env_file.to_string_lossy()).as_str(),
                    &check,
                );
            }
        }
    }
    Ok(())
}
