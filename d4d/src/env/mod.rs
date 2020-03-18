use crate::project::Project;
use env::Env;
use std::fs;
use std::path::PathBuf;
use utils::error::Error;
use utils::result::Result;

pub fn env_file_name(env: &String) -> String {
    format!(".{}", env)
}

fn get_env_file(env_directory: &PathBuf, env_file: &String) -> Option<PathBuf> {
    if let Ok(mut read_dir) = fs::read_dir(env_directory) {
        return read_dir.find_map(|entry| {
            if let Ok(entry) = entry {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if &file_name == env_file {
                        return Some(entry.path());
                    }
                }
            }
            None
        });
    }
    None
}

fn get_public(project: &Project, env_file: &String) -> Option<PathBuf> {
    if let Some(env_directory) = project.public_env_directory() {
        get_env_file(&env_directory, env_file)
    } else {
        None
    }
}

fn get_private(project: &Project, env_file: &String) -> Option<PathBuf> {
    if let Some(env_directory) = project.private_env_directory() {
        get_env_file(&env_directory, env_file)
    } else {
        None
    }
}

fn read_env_file(env_file: &PathBuf) -> Result<Env> {
    match Env::from_file(env_file) {
        Ok(env) => return Ok(env),
        Err(err) => {
            return Err(Error::wrap(
                format!("fail to read env {}", env_file.to_string_lossy()),
                Error::from(err),
            ))
        }
    }
}

pub fn get(project: &Project, env: &String) -> Result<Env> {
    let env_file_name = env_file_name(&env);
    match (
        get_public(&project, &env_file_name),
        get_private(&project, &env_file_name),
    ) {
        (Some(public_env_file), Some(private_env_file)) => Err(Error::new(format!(
            "env {} project {} has a conflit between public and private env file {} {}",
            env,project,
            public_env_file.to_string_lossy(),
            private_env_file.to_string_lossy(),
        ))),
        (Some(public_env_file), None) => read_env_file(&public_env_file),
        (None, Some(private_env_file)) => read_env_file(&private_env_file),
        (None, None) => Err(Error::new(format!(
            "env {} project {} do not provide of local.public_env_directory or global.private_env_directory",
            env,
            project
        ))),
    }
}
