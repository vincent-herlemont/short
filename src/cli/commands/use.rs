use anyhow::{Context, Result};
use clap::ArgMatches;

use crate::cfg::Cfg;
use crate::cli::cfg::get_cfg;
use crate::cli::settings::{Settings};
use crate::cli::terminal::message::success;

pub fn r#use(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let mut settings: Settings = (&cfg).into();
    if app.is_present("environment") {
        if let Some(setup) = app.value_of_lossy("setup_or_environment") {
            settings.set_setup(setup.to_string());
        }
        if let Some(env) = app.value_of_lossy("environment") {
            settings.set_env(env.to_string());
        }
    } else {
        if let Some(setup_or_env) = app.value_of_lossy("setup_or_environment") {
            if settings.env().is_ok() {
                settings.set_env(setup_or_env.to_string());
            } else {
                settings.set_setup(setup_or_env.to_string());
            }
        }
    }

    if app.is_present("unset") {
        unuse_workflow(&cfg)?;
        cfg.save()?;
        success("unset current setup");
    } else {
        use_workflow(&cfg, &settings)?;
        cfg.save()?;
        success(format!("your current setup is `{}`", settings).as_str());
    }

    Ok(())
}

pub fn unuse_workflow(cfg: &Cfg) -> Result<()> {
    let global_project = cfg.current_project()?;
    let mut global_project = global_project.borrow_mut();
    global_project.unset_current_setup();
    Ok(())
}

pub fn use_workflow(cfg: &Cfg, settings: &Settings) -> Result<()> {
    let setup_name = settings.setup()?;
    let setup = cfg.current_setup(setup_name)?;
    let global_project = cfg.current_project()?;
    let mut global_project = global_project.borrow_mut();
    let setup_name = settings.setup()?;
    global_project.set_current_setup_name(setup_name.to_owned());
    if let Ok(env_name) = settings.env() {
        setup
            .env_file(env_name)
            .context(format!("fail to found env {:?}", env_name))?;
        global_project.set_current_env_name(env_name.to_owned());
    }

    Ok(())
}
