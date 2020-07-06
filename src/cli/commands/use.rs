use anyhow::{Context, Result};
use clap::ArgMatches;

use crate::cfg::Cfg;
use crate::cli::cfg::get_cfg;
use crate::cli::settings::{get_settings, Settings};
use crate::cli::terminal::message::success;

pub fn r#use(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app, &cfg);

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
