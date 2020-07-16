use crate::cfg::Setup;
use crate::cli::commands::{sync_workflow, SyncSettings};
use crate::cli::settings::Settings;
use crate::cli::terminal::emoji;
use crate::env_file::Env;
use anyhow::Result;
use clap::ArgMatches;

pub fn selected_envs(app: &ArgMatches, setup: &Setup, settings: &Settings) -> Result<Vec<Env>> {
    let mut selected_env_names = vec![];

    if let Ok(env_name) = settings.env() {
        selected_env_names.push(env_name.to_owned());
    }
    if let Some(mut envs) = app.values_of_lossy("environments") {
        selected_env_names.append(&mut envs);
    }

    let envs: Vec<_> = setup.envs().into_iter().filter_map(|r| r.ok()).collect();
    let recent_env = Env::recent(&envs)?;
    let sync_settings = SyncSettings::new(app);
    let mut envs = sync_workflow(recent_env, envs, sync_settings)?;
    envs.sort();
    let envs: Vec<_> = envs
        .into_iter()
        .filter(|env| {
            if let Ok(name) = env.name() {
                selected_env_names
                    .iter()
                    .find(|seleted_env_name| &name == *seleted_env_name)
                    .is_some()
            } else {
                false
            }
        })
        .collect();

    if envs.is_empty() {
        bail!(
            r#"no env founded or selected
{0} you can set a current env with the command \"short use <setup> <environment>\".
{0} you can use \"-e <env> [<env>...] \" argument."#,
            emoji::RIGHT_POINTER
        )
    }

    Ok(envs)
}
