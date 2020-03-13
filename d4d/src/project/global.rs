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
fn read_global_file<P: AsRef<Path>>(home: P) -> Result<GlobalProjects> {
    let path = global_file_path(home);
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    serde_yaml::from_reader(buf).map_err(|e| Error::from(e))
}

/// Create or overwrite project config file.
#[allow(dead_code)]
fn save_global_file<P: AsRef<Path>>(home: P, projects: &GlobalProjects) -> Result<()> {
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
struct GlobalProject {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    current_env: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    git_secret_repo: Option<String>,
}

impl GlobalProject {
    #[allow(dead_code)]
    fn new<S: AsRef<str>>(name: S) -> GlobalProject {
        GlobalProject {
            name: String::from(name.as_ref()),
            path: None,
            current_env: None,
            git_secret_repo: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalProjects {
    #[serde(skip)]
    home_dir: PathBuf,

    #[serde(rename = "projects")]
    all: Vec<GlobalProject>,
}

impl GlobalProjects {
    pub fn new<P: AsRef<Path>>(home_dir: P) -> Result<GlobalProjects> {
        let home_dir = home_dir.as_ref().to_path_buf();
        match read_global_file(&home_dir) {
            Ok(global_file) => Ok(global_file),
            Err(_) => {
                let global_directory = global_directory_path(&home_dir);
                if !global_directory.exists() {
                    match create_global_directory(&home_dir) {
                        Ok(_) => (),
                        Err(err) => {
                            return Err(Error::wrap(
                                format!(
                                    "fail to create configuration directory {}",
                                    global_directory.to_string_lossy()
                                ),
                                err,
                            ));
                        }
                    }
                }
                // TODO: match for create err only if file does not exist.
                let global_projects = GlobalProjects {
                    home_dir: home_dir.to_owned(),
                    all: vec![],
                };
                match save_global_file(&home_dir, &global_projects) {
                    Ok(_) => Ok(global_projects),
                    Err(err) => Err(Error::wrap(
                        format!(
                            "fail to create global file to {}",
                            home_dir.to_string_lossy()
                        ),
                        err,
                    )),
                }
            }
        }
    }

    pub fn add<N, P>(&mut self, name: N, path: P) -> Result<()>
    where
        N: AsRef<str>,
        P: AsRef<Path>,
    {
        let path = PathBuf::from(path.as_ref())
            .canonicalize()
            .map_err(|e| Error::wrap("fail to get absolute path of : {}", Error::from(e)))?;

        self.all.push(GlobalProject {
            name: String::from(name.as_ref()),
            path: Some(path),
            current_env: None,
            git_secret_repo: None,
        });

        if let Err(err) = save_global_file(&self.home_dir, self) {
            Err(Error::wrap(
                format!(
                    "fail to save global file to : {}",
                    self.home_dir.to_string_lossy(),
                ),
                err,
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::project::global::{
        create_global_directory, global_file_path, read_global_file, save_global_file,
        GlobalProject, GlobalProjects,
    };
    use insta::assert_debug_snapshot;
    use std::collections::HashMap;
    use std::fs::read_to_string;
    use std::path::PathBuf;
    use utils::asset::Assets;
    use utils::test::before;
    use walkdir::WalkDir;

    #[test]
    fn test_save_global_file() {
        let config = before("test_save_global_file", Assets::All(HashMap::new()));
        let projects = GlobalProjects {
            home_dir: PathBuf::new(),
            all: vec![GlobalProject::new("project_1")],
        };
        let r = create_global_directory(&config.tmp_dir);
        assert!(r.is_ok());
        let r = save_global_file(&config.tmp_dir, &projects);
        assert!(r.is_ok());
        let r = read_to_string(global_file_path(&config.tmp_dir)).unwrap();
        assert_eq!(r, String::from("---\nprojects:\n  - name: project_1"));

        // Overwrite
        let projects = GlobalProjects {
            home_dir: PathBuf::new(),
            all: vec![],
        };
        let r = save_global_file(&config.tmp_dir, &projects);
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
      path: /todo/plop/
      current_env: dev
    - name: project_4
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
