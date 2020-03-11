use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use utils::error::Error;
use utils::result::Result;

#[allow(dead_code)]
const PROJECT_FILE_NAME: &'static str = "d4d.yaml";

#[allow(dead_code)]
fn local_file_path<P: AsRef<Path>>(root: P) -> PathBuf {
    root.as_ref().join(PROJECT_FILE_NAME)
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn read_local_file<P: AsRef<Path>>(root: P) -> Result<LocalProjects> {
    let file = OpenOptions::new().read(true).open(root)?;
    let buf = BufReader::new(file);
    serde_yaml::from_reader(buf).map_err(|e| Error::from(e))
}

#[derive(Serialize, Deserialize, Debug)]
struct LocalProject {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    template_path: Option<String>,
}

impl LocalProject {
    #[allow(dead_code)]
    fn new<S: AsRef<str>>(name: S) -> LocalProject {
        LocalProject {
            name: String::from(name.as_ref()),
            template_path: None,
        }
    }
}

impl Display for LocalProject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// TODO: implement Display trait
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalProjects {
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
        match read_local_file(&current_dir) {
            Ok(local_projects) => Ok(local_projects),
            Err(_) => {
                // TODO: match for create err only if file does not exist.
                let local_projects = LocalProjects { all: vec![] };
                match save_local_file(&current_dir, &local_projects) {
                    Ok(_) => Ok(local_projects),
                    Err(err) => Err(Error::wrap(
                        format!(
                            "fail to create local file {}",
                            current_dir.as_ref().to_string_lossy()
                        ),
                        err,
                    )),
                }
            }
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
        let config = before("test_save_local_file", Assets::All(assets));
        let local_projects = read_local_file(local_file_path(&config.tmp_dir)).unwrap();
        assert_debug_snapshot!(local_projects);
    }

    #[test]
    fn test_save_local_file() {
        let config = before("test_save_local_file", Assets::All(HashMap::new()));
        let local_projects = LocalProjects {
            all: vec![LocalProject::new("test_1")],
        };
        let r = save_local_file(&config.tmp_dir, &local_projects);
        assert!(r.is_ok());
        let content = read_to_string(local_file_path(&config.tmp_dir)).unwrap();
        assert_eq!(content, String::from("---\nprojects:\n  - name: test_1"));

        // Overwrite
        let local_projects = LocalProjects { all: vec![] };
        let r = save_local_file(&config.tmp_dir, &local_projects);
        assert!(r.is_ok());
        let content = read_to_string(local_file_path(&config.tmp_dir)).unwrap();
        assert_eq!(content, String::from("---\nprojects: []"));
    }
}
