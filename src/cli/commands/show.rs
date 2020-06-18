use crate::cfg::Cfg;
use crate::cli::cfg::get_cfg;
use crate::cli::settings::Settings;
use crate::cli::terminal::message::{bad_info, good_info};
use anyhow::Result;
use clap::ArgMatches;

fn cfg() -> Result<Settings> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let settings: Settings = (&cfg).into();
    Ok(settings)
}

pub fn show(args: &ArgMatches) -> Result<()> {
    if args.is_present("display_setup") {
        if let Ok(settings) = cfg() {
            if let Ok(env) = settings.setup() {
                print!("{}", env);
            }
        }
        return Ok(());
    }

    if args.is_present("display_env") {
        if let Ok(settings) = cfg() {
            if let Ok(env) = settings.env() {
                print!("{}", env);
            }
        }
        return Ok(());
    }

    let settings = cfg()?;
    terminal(settings)?;

    Ok(())
}

fn terminal(settings: Settings) -> Result<()> {
    match (settings.setup(), settings.env()) {
        (Ok(setup), Ok(env)) => {
            good_info(format!("your current setup is {:?}:{:?}", setup, env).as_str())
        }
        (Err(_), Ok(env)) => bad_info(
            format!(
                "no setup is configured with {0:?} env . You can use \"short use {0:?} <env>\"",
                env
            )
            .as_str(),
        ),
        (Ok(setup), Err(_)) => bad_info(
            format!(
                "no env is configured for {:?} . You can use \"short use <setup> <env>\"",
                setup
            )
            .as_str(),
        ),
        (Err(_), Err(_)) => {
            bad_info("no setup is configured. You can use \"short use <setup> <env>\"")
        }
    }
    Ok(())
}
