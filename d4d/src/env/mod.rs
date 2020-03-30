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

fn get_public(path: &Option<PathBuf>, env_file: &String) -> Option<PathBuf> {
    if let Some(env_directory) = path {
        get_env_file(&env_directory, env_file)
    } else {
        None
    }
}

fn get_private(path: &Option<PathBuf>, env_file: &String) -> Option<PathBuf> {
    if let Some(env_directory) = path {
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
    let env_public_path = project.public_env_directory_abs().ok();
    let env_private_path = project.private_env_directory_abs().ok();
    match (
        get_public(&env_public_path, &env_file_name),
        get_private(&env_private_path, &env_file_name),
    ) {
        (Some(_), Some(_)) => Err(Error::new(format!(
            r#"environment {} is on conflit for {} : two versions of {} exists
please delete one of theses :
   - {}
   - {}
"#,
            env,
            project.name(),
            env_file_name,
            env_public_path.unwrap().to_string_lossy(),
            env_private_path.unwrap().to_string_lossy(),
        ))),
        (Some(public_env_file), None) => read_env_file(&public_env_file),
        (None, Some(private_env_file)) => read_env_file(&private_env_file),
        (None, None) => Err(Error::new(format!(
            r#"environment {} is not available for {} : {} not found
 - please check your env directories
"#,
            env,
            project.name(),
            env_file_name
        ))),
    }
}
