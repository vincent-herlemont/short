use serde::{Deserialize, Serialize};
use std::fs::{create_dir, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use utils::error::Error;
use utils::result::Result;

#[allow(dead_code)]
const PROJECT_CONFIG_DIRECTORY: &'static str = ".d4d";
#[allow(dead_code)]
const PROJECT_CONFIG_FILE_NAME: &'static str = "projects.yaml";

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
fn project_config_file_path<P: AsRef<Path>>(home: P) -> PathBuf {
    project_config_directory_path(home).join(PROJECT_CONFIG_FILE_NAME)
}

#[allow(dead_code)]
fn load_project_config_file<P: AsRef<Path>>(home: P) -> Result<Projects> {
    let project_config = project_config_file_path(home);
    let file = File::open(project_config)?;
    let buf = BufReader::new(file);
    serde_yaml::from_reader(buf).map_err(|e| Error::from(e))
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Projects {
    #[serde(rename = "projects")]
    all: Vec<Project>,
}

#[cfg(test)]
mod tests {
    use crate::project::{create_project_config_directory, load_project_config_file, Projects};
    use insta::assert_debug_snapshot;
    use serde_yaml;
    use std::collections::HashMap;
    use utils::test::before;
    use walkdir::WalkDir;

    #[test]
    fn test_load_project_config_file() {
        let mut assets: HashMap<&str, &str> = HashMap::new();
        assets.insert(
            "assets/.d4d/projects.yaml",
            r#"
projects: []
"#,
        );
        let config = before("test_load_project_config_directory", &assets);
        let projects = load_project_config_file(&config.tmp_dir);
        assert_debug_snapshot!(projects);
    }

    #[test]
    fn test_create_project_config_directory() {
        let config = before("create_project_config_directory", &HashMap::new());
        let r = create_project_config_directory(&config.tmp_dir);
        assert_debug_snapshot!(r);
        let r: Vec<_> = WalkDir::new(&config.tmp_dir).into_iter().collect();
        assert_debug_snapshot!(r.iter().count());
        let r = create_project_config_directory(&config.tmp_dir);
        assert_debug_snapshot!(r);
    }

    #[test]
    fn test_serialize_projects() {
        let root = Projects { all: vec![] };
        let s = serde_yaml::to_string(&root).unwrap();
        assert_debug_snapshot!(&s);
    }
}
