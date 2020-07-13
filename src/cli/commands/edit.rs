use colored::*;
use std::env;
use std::process::Command;

use anyhow::{Context, Result};
use clap::ArgMatches;

use crate::cli::cfg::get_cfg;
use crate::cli::error::CliError;
use crate::cli::settings::get_settings;
use crate::cli::terminal::message::success;

use super::sync::{sync_workflow, SyncSettings};

pub fn env_edit(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app, &cfg);
    let env_name = settings.env()?;

    let editor = app.value_of("editor");
    let sync_settings = SyncSettings::new(app);

    let setup = cfg.current_setup(settings.setup()?)?;
    let env_file = setup.env_file(env_name)?;

    let command = |editor: &str| Command::new(editor).arg(&env_file).status();
    let exist_code = if let Some(editor) = editor {
        command(editor)?
    } else if let Ok(editor) = env::var("EDITOR") {
        command(editor.as_str())?
    } else {
        open::that(&env_file)?
    };

    if exist_code.code().is_none() || exist_code.code().unwrap() > 0 {
        return Err(CliError::OpenEditorFail.into());
    }

    let env = setup
        .env(env_name)
        .context(format!("fail to check env file `{}`", env_name.bold()))?;

    success(format!("`{}` edited", env_name.bold()).as_str());

    let envs = setup.envs().into_iter().filter_map(|r| r.ok()).collect();

    sync_workflow(env, envs, sync_settings)?;

    Ok(())
}
