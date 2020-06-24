use anyhow::Result;
use clap::ArgMatches;

use crate::cli::cfg::get_cfg;
use crate::cli::settings::get_settings;
use crate::cli::terminal::message::success;

use super::r#use::use_workflow;

pub fn rename(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;

    let mut settings = get_settings(&app, &cfg);

    let last_setup_name = app.value_of("last_setup_name").unwrap();
    let new_setup_name = app.value_of("new_setup_name").unwrap();

    let setup = cfg.current_setup(&last_setup_name.to_string())?;

    setup.rename(&new_setup_name.to_string())?;

    if let Ok(current_setup_name) = settings.setup() {
        if last_setup_name == current_setup_name {
            settings.set_setup(new_setup_name.to_string());
            use_workflow(&cfg, &settings)?;
        }
    }

    cfg.save()?;

    success("setup renamed");

    Ok(())
}
