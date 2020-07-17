use crate::env_file::Env;
use crate::utils::file_time::{modification_time, ModificationTime};
use anyhow::{Context, Result};

fn env_file_time(env: &Env) -> ModificationTime {
    let file = env.file();
    modification_time(file)
}

impl Env {
    pub fn recent(envs: &Vec<Env>) -> Result<Env> {
        envs.iter()
            .fold(None, |last_env, next_env| match (last_env, next_env) {
                (None, next_env) => Some(next_env.clone()),
                (Some(last_env), next_env) => {
                    let last_env_modification_time = env_file_time(&last_env);
                    let next_env_modification_time = env_file_time(next_env);
                    if last_env_modification_time < next_env_modification_time {
                        Some((*next_env).clone())
                    } else {
                        Some(last_env)
                    }
                }
            })
            .context("fail to found the most recent env file")
    }
}
