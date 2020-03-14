use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use utils::error::Error;
use utils::result::Result;

const PROJECT_FILE_NAME: &'static str = "d4d.yaml";

fn local_file_path<P: AsRef<Path>>(root: P) -> PathBuf {
    root.as_ref().join(PROJECT_FILE_NAME)
}

fn save_local_file<P: AsRef<Path>>(root: P, local_projects: &LocalProjects) -> Result<()> {
    let path = local_file_path(root);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let buf = BufWriter::new(file);
    serde_yaml::to_writer(buf, &local_projects).map_err(|e| Error::from(e))
}

fn read_local_file<P: AsRef<Path>>(root: P) -> Result<LocalProjects> {
    let file = OpenOptions::new().read(true).open(root)?;
    let buf = BufReader::new(file);
    serde_yaml::from_reader(buf).map_err(|e| Error::from(e))
}

#[derive(Serialize, Deserialize, Debug)]
struct LocalProject {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    template_path: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    public_env_directory: Option<PathBuf>,
}

impl LocalProject {
    #[allow(dead_code)]
    fn new<S: AsRef<str>>(name: S) -> LocalProject {
        LocalProject {
            name: String::from(name.as_ref()),
            template_path: None,
            public_env_directory: None,
        }
    }
}

impl Display for LocalProject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalProjects {
    #[serde(skip)]
    current_dir: PathBuf,

    #[serde(rename = "projects")]
    all: Vec<LocalProject>,
}

impl Display for LocalProjects {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for local_project in &self.all {
            writeln!(f, " - {}", local_project)?;
        }
        Ok(())
    }
}

impl LocalProjects {
    pub fn new<P: AsRef<Path>>(current_dir: P) -> Result<LocalProjects> {
        let current_dir = current_dir.as_ref().to_path_buf();
        match read_local_file(&current_dir) {
            Ok(local_projects) => Ok(local_projects),
            Err(_) => {
                // TODO: match for create err only if file does not exist.
                let local_projects = LocalProjects {
                    current_dir: current_dir.to_owned(),
                    all: vec![],
                };
                match save_local_file(&current_dir, &local_projects) {
                    Ok(_) => Ok(local_projects),
                    Err(err) => Err(Error::wrap(
                        format!(
                            "fail to create local file {}",
                            current_dir.to_string_lossy()
                        ),
                        err,
                    )),
                }
            }
        }
    }

    pub fn add<N, TP, PED>(
        &mut self,
        name: N,
        template_path: TP,
        public_env_directory: PED,
    ) -> Result<()>
    where
        N: AsRef<str>,
        TP: AsRef<Path>,
        PED: AsRef<Path>,
    {
        self.all.push(LocalProject {
            name: String::from(name.as_ref()),
            template_path: Some(PathBuf::from(template_path.as_ref())),
            public_env_directory: Some(PathBuf::from(public_env_directory.as_ref())),
        });

        if let Err(err) = save_local_file(&self.current_dir, self) {
            Err(Error::wrap(
                format!(
                    "fail to save local file : {}",
                    self.current_dir.to_string_lossy()
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
    use crate::project::local::{
        local_file_path, read_local_file, save_local_file, LocalProject, LocalProjects,
    };
    use insta::assert_debug_snapshot;
    use std::collections::HashMap;
    use std::fs::read_to_string;
    use std::path::PathBuf;
    use utils::asset::Assets;
    use utils::test::before;

    #[test]
    fn test_read_local_file() {
        let mut assets = HashMap::new();
        assets.insert(
            "d4d.yaml",
            r#"
projects:
    - name: test_1
    - name: test_2
      template_path: "./test_template.yaml"
        "#,
        );
        let config = before("test_save_local_file", Assets::Static(assets));
        let local_projects = read_local_file(local_file_path(&config.tmp_dir)).unwrap();
        assert_debug_snapshot!(local_projects);
    }

    #[test]
    fn test_save_local_file() {
        let config = before("test_save_local_file", Assets::Static(HashMap::new()));
        let local_projects = LocalProjects {
            current_dir: PathBuf::new(),
            all: vec![LocalProject::new("test_1")],
        };
        let r = save_local_file(&config.tmp_dir, &local_projects);
        assert!(r.is_ok());
        let content = read_to_string(local_file_path(&config.tmp_dir)).unwrap();
        assert_eq!(content, String::from("---\nprojects:\n  - name: test_1"));

        // Overwrite
        let local_projects = LocalProjects {
            current_dir: PathBuf::new(),
            all: vec![],
        };
        let r = save_local_file(&config.tmp_dir, &local_projects);
        assert!(r.is_ok());
        let content = read_to_string(local_file_path(&config.tmp_dir)).unwrap();
        assert_eq!(content, String::from("---\nprojects: []"));
    }
}
