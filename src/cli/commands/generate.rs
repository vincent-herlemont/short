use std::env::current_dir;
use std::fs::create_dir_all;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::ArgMatches;
use tempdir::TempDir;
use term_table::row::Row;
use term_table::table_cell::TableCell;

use crate::cfg::{load_local_cfg, LocalSetupCfg, SetupCfg, SetupsCfg};
use crate::cli::cfg::get_cfg;
use crate::cli::error::CliError;
use crate::cli::error::CliError::UnknownError;
use crate::cli::settings::Settings;
use crate::cli::terminal::message::success;
use crate::run_file::File;
use crate::template::Registry;

use super::new::env_new_workflow;
use super::r#use::use_workflow;

pub struct GenerateSettings {
    pub setup_name: String,
}

impl GenerateSettings {
    fn new(app: &ArgMatches) -> Self {
        Self {
            setup_name: app.value_of("setup_name").unwrap().into(),
        }
    }
}

pub fn generate(app: &ArgMatches) -> Result<()> {
    if app.is_present("list") {
        display_template_list()?;
        return Ok(());
    }

    let generate_settings = GenerateSettings::new(app);

    if app.is_present("template") {
        generate_template_workflow(app, &generate_settings)
    } else {
        generate_empty_workflow(app, &generate_settings)
    }
}

fn display_template_list() -> Result<()> {
    let registry_tmp = TempDir::new("registry")?;
    let registry = Registry::checkout(registry_tmp.path())?;
    let mut render_table = term_table::Table::new();
    render_table.separate_rows = false;
    render_table.style = term_table::TableStyle::blank();

    for template in registry.index() {
        if template.name().eq("test") {
            // Remove test repository
            continue;
        }
        render_table.add_row(Row::new(vec![
            TableCell::new(template.name()),
            TableCell::new(template.url()),
        ]));
    }

    println!("{}", render_table.to_string());

    Ok(())
}

fn generate_template_workflow(
    app: &ArgMatches,
    generate_settings: &GenerateSettings,
) -> Result<()> {
    let mut cfg = get_cfg()?;

    let setup_name: String = generate_settings.setup_name.clone();

    let template_name: String = app.value_of("template").unwrap().into();
    let target_template_directory: PathBuf = app
        .value_of("target_template_directory")
        .unwrap_or(setup_name.as_str())
        .into();
    let registry_tmp = TempDir::new("registry")?;
    let registry = Registry::checkout(registry_tmp.path())?;
    let mut template = registry.get(template_name.as_str())?;
    let temp_dir = TempDir::new(format!("generate_template_{}", template_name).as_str())?;
    template.checkout(temp_dir.path().clone().to_path_buf())?;

    // Load and add new local setup cfg
    let local_cfg = load_local_cfg(&temp_dir.path().to_path_buf())?;
    let local_cfg = local_cfg.borrow();
    let local_setups = local_cfg.get_setups();
    let local_setups = local_setups.borrow();
    let local_setup = local_setups.get(0).context("setup template not found")?;
    let mut local_setup = local_setup.borrow().clone();
    if app.is_present("target_template_directory") {
        let public_env_dir = target_template_directory.join(local_setup.public_env_dir());
        local_setup.set_public_env_dir(public_env_dir);

        let file = target_template_directory.join(local_setup.file());
        local_setup.set_file(file);
    }
    local_setup.set_name(setup_name.clone());
    cfg.add_local_setup_cfg(local_setup);
    cfg.sync_local_to_global()?; // After add new setup we need to sync for apply others actions.

    let target_dir = if app.is_present("target_template_directory") {
        create_dir_all(&target_template_directory)?;
        target_template_directory
    } else {
        current_dir()?
    };

    template.copy(target_dir).context("fail to copy files")?;

    // Retrieve the env if there is an env
    let local_setup = cfg.current_setup(&setup_name)?;
    let env = local_setup.envs().into_iter().find_map(|env| env.ok());

    // Use new setup and env
    let mut settings = Settings::new();
    settings.set_setup(setup_name.clone());
    if let Some(env) = env.as_ref() {
        settings.set_env(env.name()?);
    }
    use_workflow(&cfg, &settings)?;
    cfg.save()?;

    let env_str = env
        .as_ref()
        .map(|env| env.name())
        .unwrap_or(Ok("".to_string()));
    success(format!("generate setup `{}`:`{}`", setup_name, env_str?).as_str());

    Ok(())
}

fn generate_empty_workflow(app: &ArgMatches, generate_settings: &GenerateSettings) -> Result<()> {
    let mut cfg = get_cfg()?;
    let setup_name: String = generate_settings.setup_name.clone();
    let public_env_directory = app.value_of("public_env_directory");
    let env_name = app.value_of("env_name").unwrap().to_string();
    let setup_file = app.value_of("file").unwrap_or("run.sh").to_string();
    let setup_shebang = app.value_of("shebang").unwrap_or("#!/bin/bash").to_string();
    let private = app.is_present("private");

    let setup_file = PathBuf::from(setup_file);

    let mut local_setup_cfg = LocalSetupCfg::new(setup_name.clone(), setup_file.clone());

    // New script file
    let mut file = File::new(setup_file.clone(), setup_shebang.to_string());
    {
        let array_vars = local_setup_cfg.array_vars().unwrap_or_default();
        let vars = local_setup_cfg.vars().unwrap_or_default();
        if let Some(public_env_directory) = public_env_directory {
            local_setup_cfg.set_public_env_dir(public_env_directory.into());
        }
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
