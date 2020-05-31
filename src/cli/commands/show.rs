use crate::cli::cfg::get_cfg;

use crate::cli::terminal::message::{bad_info, good_info};

use anyhow::{Result};


pub fn show() -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let current_project = cfg.current_project()?;
    let current_project = current_project.borrow();

    match (
        current_project.current_setup_name(),
        current_project.current_env_name(),
    ) {
        (Some(setup), Some(env)) => {
            good_info(format!("your current setup is {:?}:{:?}", setup, env).as_str())
        }
        (None, Some(env)) => bad_info(
            format!(
                "no setup is configured with {0:?} env . You can use \"short use {0:?} <env>\"",
                env
            )
            .as_str(),
        ),
        (Some(setup), None) => bad_info(
            format!(
                "no env is configured for {:?} . You can use \"short use <setup> <env>\"",
                setup
            )
            .as_str(),
        ),
        (None, None) => bad_info("no setup is configured. You can use \"short use <setup> <env>\""),
    }

    Ok(())
}
