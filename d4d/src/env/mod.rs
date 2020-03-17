use crate::project::{Project};
use env::Env;
use std::fs;
use utils::error::Error;
use utils::result::Result;

pub fn env_file_name(env: &String) -> String {
    format!(".{}", env)
}

pub fn get(project: &Project, env: &String) -> Result<Env> {
    let env_file_name = env_file_name(&env);
    if let Some(public_env_directory) = project.public_env_directory() {
        let found_env = fs::read_dir(public_env_directory)?.find_map(|entry| {
            if let Ok(entry) = entry {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if file_name == env_file_name {
                        return Some(entry.path());
                    }
                }
            }
            None
        });
        if let Some(env_file) = found_env {
            match Env::from_file(env_file) {
                Ok(env) => return Ok(env),
                Err(err) => {
                    return Err(Error::wrap(
                        format!("fail to retrieve env {} for project {}", env, project),
                        Error::from(err),
                    ))
                }
            }
        } else {
            Err(Error::new(format!(
                "cannot found env {} for project {}",
                env, project
            )))
        }
    } else {
        Err(Error::new(format!(
            "project {} do not provide of public_env_directory",
            project
        )))
    }
}
