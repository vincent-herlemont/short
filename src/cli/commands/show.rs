use crate::cli::cfg::get_cfg;
use crate::cli::settings::Settings;
use crate::cli::terminal::message::{bad_info, good_info};
use anyhow::Result;

pub fn show() -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings: Settings = (&cfg).into();

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
