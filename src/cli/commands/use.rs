use crate::cli::cfg::get_cfg;
use crate::cli::settings::get_settings;
use crate::cli::terminal::message::success;

use anyhow::{Context, Result};
use clap::ArgMatches;

pub fn r#use(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app);
    let setup_name = settings.setup()?;
    let env = settings.env()?;

    // Check if setup exist
    let setup = cfg
        .current_setup(setup_name)?
        .context(format!("fail to found setup {:?}", setup_name))?;
    // Check if env exist and loadable
    if !setup.env_exist(env) {
        return Err(anyhow!("fail to found env {:?}", env));
    }

    {
        let global_project = cfg.current_project()?;
        let mut global_project = global_project.borrow_mut();
        global_project.set_current_setup_name(setup_name.to_owned());
        global_project.set_current_env_name(env.to_owned());
    }

    cfg.save()?;

    success(format!("your current setup is {:?}:{:?}", setup_name, env).as_str());

    Ok(())
}
