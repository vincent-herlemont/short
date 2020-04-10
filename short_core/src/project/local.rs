use crate::project::provider::{AwsCfg, ProviderCfg};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use short_utils::error::Error;

use short_utils::result::Result;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

const PROJECT_FILE_NAME: &'static str = "short.yaml";

fn local_file_path<P: AsRef<Path>>(root: P) -> PathBuf {
    root.as_ref().join(PROJECT_FILE_NAME)
}

fn local_file_exist<P: AsRef<Path>>(root: P) -> Result<()> {
    let path = local_file_path(root);
    if path.exists() {
        Err(Error::new(format!(
            "project file exists {}",
            path.to_string_lossy()
        )))
    } else {
        Ok(())
    }
}

fn save_local_file<P: AsRef<Path>>(root: P, local_projects: &LocalProjects) -> Result<()> {
    let path = local_file_path(root);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    let buf = BufWriter::new(file);
    serde_yaml::to_writer(buf, &local_projects).map_err(|err| {
        Error::wrap(
            format!("fail to save project file {}", path.to_string_lossy()),
            Error::from(err),
        )
    })
}

/// Find local file recursively in parent paths
fn find_current_dir<P: AsRef<Path>>(current_dir: P) -> Result<PathBuf> {
    let current_dir = current_dir.as_ref().to_path_buf();
    let local_file = local_file_path(&current_dir);
    if !local_file.exists() {
        let parent = current_dir
            .parent()
            .ok_or(Error::new("project file not found"))?
            .to_path_buf();
        find_current_dir(parent)
    } else {
        Ok(current_dir)
    }
}

fn read_local_file<P: AsRef<Path>>(root: P) -> Result<LocalProjects> {
    let current_dir = find_current_dir(&root)?;
    let local_file = local_file_path(&current_dir);
    let file = OpenOptions::new()
        .read(true)
        .open(&local_file)
        .map_err(|err| {
            Error::wrap(
                format!("fail to open project file {}", local_file.to_string_lossy()),
                Error::from(err),
            )
        })?;
    let buf = BufReader::new(file);
    serde_yaml::from_reader(buf)
        .map_err(|err| {
            Error::wrap(
                format!("fail to read project file {}", local_file.to_string_lossy()),
                Error::from(err),
            )
        })
        .map(|local_projects: LocalProjects| LocalProjects {
            current_dir,
            ..local_projects
        })
}

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct LocalProject {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    public_env_directory: Option<PathBuf>,

    provider: ProviderCfg,
}

impl LocalProject {
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn public_env_directory(&self) -> Result<PathBuf> {
        self.public_env_directory.clone().ok_or(Error::new(format!(
            "public_env_directory not found for {}",
            self.name()
        )))
    }

    pub fn provider(&self) -> &ProviderCfg {
        &self.provider
    }
}

impl PartialEq for LocalProject {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for LocalProject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
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
    all: Vec<Box<LocalProject>>,
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
    pub fn load<P: AsRef<Path>>(current_dir: P) -> Result<LocalProjects> {
        let current_dir = current_dir.as_ref().to_path_buf();
        let local_projects = read_local_file(&current_dir)?;
        local_projects.has_unique_project()?;
        Ok(local_projects)
    }

    fn has_unique_project(&self) -> Result<()> {
        let mut uniq = HashSet::new();
        let b = self.all.iter().all(|x| uniq.insert(x));
        if b {
            Ok(())
        } else {
            Err(Error::new(
                "some project(s) are duplicate on configuration file",
            ))
        }
    }

    pub fn new<P: AsRef<Path>>(current_dir: P) -> Result<LocalProjects> {
        let local_projects = LocalProjects {
            current_dir: current_dir.as_ref().to_owned(),
            all: vec![],
        };
        local_file_exist(&current_dir)?;
        save_local_file(&current_dir, &local_projects)?;
        Ok(local_projects)
    }

    pub fn get_all(&self) -> &Vec<Box<LocalProject>> {
        self.all.as_ref()
    }

    pub fn add<N, PED>(
        &mut self,
        name: N,
        public_env_directory: PED,
        provider: ProviderCfg,
    ) -> Result<&LocalProject>
    where
        N: AsRef<str>,
        PED: AsRef<Path>,
    {
        let name = name.as_ref().to_string();
        if let Some(_) = self.get(&name) {
            return Err(Error::new(format!("project {} already exists", &name)));
        }

        let local_project = Box::new(LocalProject {
            name: name.clone(),
            public_env_directory: Some(PathBuf::from(public_env_directory.as_ref())),
            provider,
        });
        self.all.push(local_project);

        if let Err(err) = save_local_file(&self.current_dir, self) {
            Err(Error::wrap(
                format!(
                    "fail to save local file : {}",
                    self.current_dir.to_string_lossy()
                ),
                err,
            ))
        } else {
            Ok(self.get(name).unwrap())
        }
    }

    pub fn get<P: AsRef<str>>(&self, project_name: P) -> Option<&LocalProject> {
        self.all.iter().find_map(|local_project| {
            if local_project.name == project_name.as_ref() {
                Some(local_project.as_ref())
            } else {
                None
            }
        })
    }

    pub fn fake() -> Self {
        let mut aws_cfg_1 = AwsCfg::new();
        aws_cfg_1.set_template_path("./project_test.tpl");

        let mut aws_cfg_2 = AwsCfg::new();
        aws_cfg_2.set_template_path("./project_test_bis.tpl");

        let mut aws_cfg_3 = AwsCfg::new();
        aws_cfg_3.set_template_path("./project_test_bis.tpl");

        Self {
            current_dir: PathBuf::from("/path/to/local"),
            all: vec![
                Box::new(LocalProject {
                    name: String::from("project_test"),
                    public_env_directory: None,
                    provider: ProviderCfg::ConfAws(aws_cfg_1),
                }),
                Box::new(LocalProject {
                    name: String::from("project_test_bis"),
                    public_env_directory: None,
                    provider: ProviderCfg::ConfAws(aws_cfg_2),
                }),
                Box::new(LocalProject {
                    name: String::from("only_local_project_test"),
                    public_env_directory: None,
                    provider: ProviderCfg::ConfAws(aws_cfg_3),
                }),
            ],
        }
    }

    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }
}

#[cfg(test)]
mod tests {
    use crate::project::local::{local_file_path, read_local_file, save_local_file, LocalProjects};
    use insta::assert_yaml_snapshot;
    use short_utils::asset::Assets;
    use short_utils::test::before;
    use std::collections::HashMap;
    use std::fs::read_to_string;
    use std::path::PathBuf;

    #[test]
    fn test_read_local_file() {
        let mut assets = HashMap::new();
        assets.insert(
            "short.yaml",
            r#"
projects:
    - name: test_1
      provider:
        name: aws
    - name: test_2
      provider:
        name: aws
        template_path: "./test_template.yaml"
        "#,
        );
        let config = before("test_save_local_file", Assets::Static(assets));
        let local_projects = read_local_file(&config.tmp_dir).unwrap();
        assert_eq!(&local_projects.current_dir, &config.tmp_dir);
        assert_yaml_snapshot!(local_projects);
    }

    #[test]
    fn test_save_local_file() {
        let config = before("test_save_local_file", Assets::Static(HashMap::new()));
        let local_projects = LocalProjects::fake();
        let r = save_local_file(&config.tmp_dir, &local_projects);
        assert!(r.is_ok());
        let content = read_to_string(local_file_path(&config.tmp_dir)).unwrap();
        assert_eq!(
            content,
            r#"---
projects:
  - name: project_test
    provider:
      name: aws
      template_path: "./project_test.tpl"
  - name: project_test_bis
    provider:
      name: aws
      template_path: "./project_test_bis.tpl"
  - name: only_local_project_test
    provider:
      name: aws
      template_path: "./project_test_bis.tpl""#
        );

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
