use anyhow::Result;
use clap::ArgMatches;

use crate::cli::cfg::get_cfg;
use crate::cli::settings::Settings;
use crate::cli::terminal::message::{bad_info, good_info};

pub const DEFAULT_SHOW_FORMAT: &'static str = "[{setup}:{env}]";

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
    } else if args.is_present("display_env") {
        if let Ok(settings) = cfg() {
            if let Ok(env) = settings.env() {
                print!("{}", env);
            }
        }
    } else if args.is_present("format") {
        if let Ok(settings) = cfg() {
            let format = args
                .value_of_lossy("format")
                .map(|c| c.into_owned())
                .unwrap_or(DEFAULT_SHOW_FORMAT.into());

            let format = format.replace("{setup}", settings.setup().unwrap_or(&"".into()));
            let format = format.replace("{env}", settings.env().unwrap_or(&"".into()));

            print!("{}", format);
        }
    } else {
        let settings = cfg()?;
        terminal(settings)?;
    }

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
