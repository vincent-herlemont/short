use anyhow::{Context, Result};
use clap::ArgMatches;

use crate::cli::cfg::get_cfg;
use crate::cli::settings::get_settings;
use crate::run_file::{generate_env_vars, run_as_stream};

pub fn run(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app, &cfg);

    let setup_name = settings.setup()?;
    let env = settings.env()?;
    let setup = cfg.current_setup(setup_name)?;

    let script_file = setup.local_cfg_run_file()?;
    let env = setup.env(&env)?;

    let local_setup = setup.local_setup().unwrap();
    let local_setup = local_setup.borrow();
    let array_vars = local_setup.array_vars().unwrap_or_default();
    let vars = local_setup.vars().unwrap_or_default();
    drop(local_setup);

    let env_vars = generate_env_vars(&env, array_vars.borrow(), vars.borrow())?;

    run_as_stream(&script_file, &env_vars)
        .context(format!("fail to run {:?} with env {:?}", script_file, env))?;

    Ok(())
}
