use super::env_new::env_new_workflow;
use super::r#use::use_workflow;
use crate::cfg::LocalSetupCfg;
use crate::cli::cfg::get_cfg;
use crate::cli::error::CliError;
use crate::cli::error::CliError::UnknownError;
use crate::cli::settings::Settings;
use crate::cli::terminal::message::success;
use crate::run_file::File;
use crate::template::Registry;
use anyhow::{Context, Result};
use clap::ArgMatches;

use std::env::{current_dir};
use std::path::PathBuf;
use tempdir::TempDir;

pub fn generate(app: &ArgMatches) -> Result<()> {
    if app.is_present("template") {
        generate_template_workflow(app)
    } else {
        generate_empty_workflow(app)
    }
}

fn generate_template_workflow(app: &ArgMatches) -> Result<()> {
    let _cfg = get_cfg()?;

    let template_name: String = app.value_of("template").unwrap().into();
    let registry = Registry::new();
    let mut template = registry.get(template_name.as_str())?;
    let temp_dir = TempDir::new(format!("generate_template_{}", template_name).as_str())?;
    template.checkout(temp_dir.path().clone().to_path_buf())?;
    let target_dir = current_dir()?;
    template.copy(target_dir).context("fail to copy files")?;
    Ok(())
}

fn generate_empty_workflow(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    let setup_name: String = app.value_of("setup_name").unwrap().into();
    let env_name: String = app.value_of("env_name").unwrap().into();
    let setup_file = app.value_of("file").unwrap();
    let setup_shebang = app.value_of("shebang").unwrap();
    let private = app.is_present("private");

    let setup_file = PathBuf::from(setup_file);

    let local_setup_cfg = LocalSetupCfg::new(setup_name.clone(), setup_file.clone());

    // New script file
    let mut file = File::new(setup_file.clone(), setup_shebang.to_string());
    {
        let array_vars = local_setup_cfg.array_vars().unwrap_or_default();
        let vars = local_setup_cfg.vars().unwrap_or_default();
        file.generate(array_vars.borrow(), vars.borrow())?;
    }
    file.save()?;
    cfg.add_local_setup_cfg(local_setup_cfg);
    cfg.sync_local_to_global()?;

    // Add env
    let env = match env_new_workflow(&cfg, &setup_name, &env_name, &private) {
        Ok(env) => env,
        Err(err) => match err.downcast::<CliError>() {
            Ok(CliError::EnvFileAlreadyExists(_, env)) => Ok(env.clone()),
            Ok(err) => Err(err),
            Err(err) => Err(UnknownError(err)),
        }?,
    };

    // Use new setup and env
    let mut settings = Settings::new();
    settings.set_setup(setup_name.clone());
    settings.set_env(env_name.clone());
    use_workflow(&cfg, &settings)?;

    cfg.save()?;

    success(format!("generate setup `{}`:`{}`", setup_name, env.name()?).as_str());

    Ok(())
}
