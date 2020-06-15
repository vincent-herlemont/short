use crate::env_file::Env;
use anyhow::{Context, Result};
use filetime::FileTime;
use std::fs;

type ModificationTime = FileTime;
type CreateTime = FileTime;

fn env_file_time(env: &Env) -> (ModificationTime, Option<CreateTime>) {
    let file = env.file();
    let metadata = fs::metadata(file).unwrap();
    (
        FileTime::from_last_modification_time(&metadata),
        FileTime::from_creation_time(&metadata),
    )
}

impl Env {
    pub fn recent(envs: &Vec<Env>) -> Result<Env> {
        envs.iter()
            .fold(None, |last_env, next_env| match (last_env, next_env) {
                (None, next_env) => Some(next_env.clone()),
                (Some(last_env), next_env) => {
                    let (last_env_modification_time, last_env_create_time) =
                        env_file_time(&last_env);
                    let (next_env_modification_time, next_env_create_time) =
                        env_file_time(next_env);
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
