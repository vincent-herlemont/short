use serde::{Deserialize, Serialize};
use std::fs::{create_dir, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use utils::error::Error;
use utils::result::Result;

#[allow(dead_code)]
const PROJECT_CONFIG_DIRECTORY: &'static str = ".d4d";
#[allow(dead_code)]
const PROJECT_CURRENT_FILE_NAME: &'static str = "current.yaml";

fn project_config_directory_path<P: AsRef<Path>>(home: P) -> PathBuf {
    home.as_ref().join(PROJECT_CONFIG_DIRECTORY)
}

#[allow(dead_code)]
fn create_project_config_directory<P: AsRef<Path>>(home: P) -> Result<()> {
    let project_config_directory = project_config_directory_path(home);
    match create_dir(project_config_directory) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::from(e)),
    }
}

#[allow(dead_code)]
fn project_current_file_path<P: AsRef<Path>>(home: P) -> PathBuf {
    project_config_directory_path(home).join(PROJECT_CURRENT_FILE_NAME)
}

#[allow(dead_code)]
fn load_project_current_file<P: AsRef<Path>>(home: P) -> Result<Projects> {
    let path = project_current_file_path(home);
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    serde_yaml::from_reader(buf).map_err(|e| Error::from(e))
}

/// Create or overwrite project config file.
#[allow(dead_code)]
fn save_project_current_file<P: AsRef<Path>>(home: P, projects: Projects) -> Result<()> {
    let path = project_current_file_path(home);
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

    #[serde(skip_serializing)]
    current_env: Option<String>,

    #[serde(skip_serializing)]
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
    use crate::project::current::{
        create_project_config_directory, load_project_current_file, project_current_file_path,
        save_project_current_file, Project, Projects,
    };
    use insta::assert_debug_snapshot;
    use serde_yaml;
    use std::collections::HashMap;
    use std::fs::read_to_string;
    use utils::asset::Assets;
    use utils::test::before;
    use walkdir::WalkDir;

    #[test]
    fn test_save_project_current_file() {
        let config = before(
            "test_save_project_current_file",
            Assets::All(HashMap::new()),
        );
        let projects = Projects {
            all: vec![Project::new("project_1")],
        };
        let r = create_project_config_directory(&config.tmp_dir);
        assert!(r.is_ok());
        let r = save_project_current_file(&config.tmp_dir, projects);
        assert!(r.is_ok());
        let r = read_to_string(project_current_file_path(&config.tmp_dir)).unwrap();
        assert_eq!(r, String::from("---\nprojects:\n  - name: project_1"));
        let read_create_projects_file = load_project_current_file(&config.tmp_dir);
        assert_debug_snapshot!(read_create_projects_file);
        let projects = Projects { all: vec![] };
        let r = save_project_current_file(&config.tmp_dir, projects);
        assert!(r.is_ok());
        let r = read_to_string(project_current_file_path(&config.tmp_dir)).unwrap();
        assert_eq!(r, String::from("---\nprojects: []"));
        let read_overwrite_projects_file = load_project_current_file(&config.tmp_dir);
        assert_debug_snapshot!(read_overwrite_projects_file);
    }

    #[test]
    fn test_load_project_current_file() {
        let mut assets: HashMap<&str, &str> = HashMap::new();
        assets.insert(
            ".d4d/current.yaml",
            r#"
projects: []
"#,
        );
        let config = before("test_load_project_current_file", Assets::All(assets));
        let projects = load_project_current_file(&config.tmp_dir);
        assert_debug_snapshot!(projects);
    }

    #[test]
    fn test_create_project_config_directory() {
        let config = before(
            "create_project_config_directory",
            Assets::All(HashMap::new()),
        );
        let r = create_project_config_directory(&config.tmp_dir);
        assert!(r.is_ok());
        let r: Vec<_> = WalkDir::new(&config.tmp_dir).into_iter().collect();
        assert_debug_snapshot!(r.iter().count());
        let r = create_project_config_directory(&config.tmp_dir);
        assert!(r.is_err());
    }

    #[test]
    fn test_serialize_projects() {
        let root = Projects { all: vec![] };
        let s = serde_yaml::to_string(&root).unwrap();
        assert_debug_snapshot!(&s);
    }
}
