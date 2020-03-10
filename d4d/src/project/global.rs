use serde::{Deserialize, Serialize};
use std::fs::{create_dir, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use utils::error::Error;
use utils::result::Result;

#[allow(dead_code)]
const PROJECT_CONFIG_DIRECTORY: &'static str = ".d4d";
#[allow(dead_code)]
const PROJECT_CURRENT_FILE_NAME: &'static str = "projects.yaml";

fn global_directory_path<P: AsRef<Path>>(home: P) -> PathBuf {
    home.as_ref().join(PROJECT_CONFIG_DIRECTORY)
}

#[allow(dead_code)]
fn create_global_directory<P: AsRef<Path>>(home: P) -> Result<()> {
    let path = global_directory_path(home);
    match create_dir(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::from(e)),
    }
}

#[allow(dead_code)]
fn global_file_path<P: AsRef<Path>>(home: P) -> PathBuf {
    global_directory_path(home).join(PROJECT_CURRENT_FILE_NAME)
}

#[allow(dead_code)]
fn read_global_file<P: AsRef<Path>>(home: P) -> Result<Projects> {
    let path = global_file_path(home);
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    serde_yaml::from_reader(buf).map_err(|e| Error::from(e))
}

/// Create or overwrite project config file.
#[allow(dead_code)]
fn save_global_file<P: AsRef<Path>>(home: P, projects: Projects) -> Result<()> {
    let path = global_file_path(home);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let buf = BufWriter::new(file);
    serde_yaml::to_writer(buf, &projects)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    current_env: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    git_secret_repo: Option<String>,
}

impl Project {
    #[allow(dead_code)]
    fn new<S: AsRef<str>>(name: S) -> Project {
        Project {
            name: String::from(name.as_ref()),
            current_env: None,
            git_secret_repo: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Projects {
    #[serde(rename = "projects")]
    all: Vec<Project>,
}

#[cfg(test)]
mod tests {
    use crate::project::global::{
        create_global_directory, global_file_path, read_global_file, save_global_file, Project,
        Projects,
    };
    use insta::assert_debug_snapshot;
    use std::collections::HashMap;
    use std::fs::read_to_string;
    use utils::asset::Assets;
    use utils::test::before;
    use walkdir::WalkDir;

    #[test]
    fn test_save_global_file() {
        let config = before("test_save_global_file", Assets::All(HashMap::new()));
        let projects = Projects {
            all: vec![Project::new("project_1")],
        };
        let r = create_global_directory(&config.tmp_dir);
        assert!(r.is_ok());
        let r = save_global_file(&config.tmp_dir, projects);
        assert!(r.is_ok());
        let r = read_to_string(global_file_path(&config.tmp_dir)).unwrap();
        assert_eq!(r, String::from("---\nprojects:\n  - name: project_1"));

        // Overwrite
        let projects = Projects { all: vec![] };
        let r = save_global_file(&config.tmp_dir, projects);
        assert!(r.is_ok());
        let r = read_to_string(global_file_path(&config.tmp_dir)).unwrap();
        assert_eq!(r, String::from("---\nprojects: []"));
    }

    #[test]
    fn test_empty_read_global_file() {
        let mut assets: HashMap<&str, &str> = HashMap::new();
        assets.insert(
            ".d4d/projects.yaml",
            r#"
projects: []
"#,
        );
        let config = before("test_empty_read_global_file", Assets::All(assets));
        let projects = read_global_file(&config.tmp_dir);
        assert_debug_snapshot!(projects);
    }

    #[test]
    fn test_read_global_file() {
        let mut assets: HashMap<&str, &str> = HashMap::new();
        assets.insert(
            ".d4d/projects.yaml",
            r#"
projects:
    - name: project_1
    - name: project_2
      current_env: dev
    - name: project_3
      current_env: prod
      git_secret_repo: "git@privategit.com" 
"#,
        );
        let config = before("test_read_global_file", Assets::All(assets));
        let projects = read_global_file(&config.tmp_dir);
        assert_debug_snapshot!(projects);
    }

    #[test]
    fn test_create_global_directory() {
        let config = before("create_global_directory", Assets::All(HashMap::new()));
        let r = create_global_directory(&config.tmp_dir);
        assert!(r.is_ok());
        let r: Vec<_> = WalkDir::new(&config.tmp_dir).into_iter().collect();
        assert_eq!(r.iter().count(), 2);
        let r = create_global_directory(&config.tmp_dir);
        assert!(r.is_err());
    }
}