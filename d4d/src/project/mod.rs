use crate::project::global::{GlobalProject, GlobalProjects};
use crate::project::local::{LocalProject, LocalProjects};
use serde::export::Formatter;
use std::fmt;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use utils::error::Error;
use utils::result::Result;

pub use super::project::global::CurrentProject;
use crate::project::provider::{AwsCfg, ProviderCfg};

pub mod global;
pub mod local;
pub mod provider;

#[derive(Debug)]
pub struct Projects {
    temp_current_project: Option<CurrentProject>,
    local: LocalProjects,
    global: GlobalProjects,
}

#[derive(Debug)]
pub struct Project<'a> {
    local: &'a LocalProject,
    global: &'a GlobalProject,
}

impl<'a> Project<'a> {
    pub fn name(&self) -> String {
        self.local.name()
    }

    pub fn public_env_directory(&self) -> Result<PathBuf> {
        self.local.public_env_directory()
    }

    pub fn private_env_directory(&self) -> Result<PathBuf> {
        self.global.private_env_directory()
    }

    pub fn project_path(&self) -> Result<PathBuf> {
        self.global.path()
    }

    /// Return the absolute path of the current project
    pub fn template_file(&self) -> Result<PathBuf> {
        let project_path = self.project_path()?;
        let template_path = self.local.provider().aws()?.template_path()?;
        Ok(project_path.join(template_path))
    }
}

impl<'a> Display for Project<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "- {}", self.name())
    }
}

impl Projects {
    pub fn init<CD, HD>(current_dir: CD, home_dir: HD) -> Result<Projects>
    where
        CD: AsRef<Path>,
        HD: AsRef<Path>,
    {
        match (
            LocalProjects::new(current_dir),
            GlobalProjects::new(home_dir),
        ) {
            (Ok(local), Ok(global)) => Ok(Projects {
                local,
                global,
                temp_current_project: None,
            }),
            (Err(err), Ok(_)) => Err(Error::from(err)),
            (Ok(_), Err(err)) => Err(Error::from(err)),
            (Err(err), Err(_)) => Err(Error::from(err)),
        }
    }

    pub fn add<N, P>(&mut self, project_name: N, template_path: P) -> Result<()>
    where
        N: AsRef<str>,
        P: AsRef<Path>,
    {
        // TODO : move template
        let template_path = template_path.as_ref();
        let public_env_directory = template_path.parent().ok_or(format!(
            "fail to get directory of template : {}",
            template_path.to_string_lossy()
        ))?;
        let mut aws_cfg = AwsCfg::new("us-east-1");
        aws_cfg.set_template_path(template_path);
        self.local.add(
            &project_name,
            public_env_directory,
            ProviderCfg::ConfAws(aws_cfg),
        )?;
        self.global.add(&project_name, public_env_directory)?;
        Ok(())
    }

    pub fn found<P: AsRef<str>>(&self, project_name: P) -> Result<Project> {
        if let (Some(global), Some(local)) = (
            self.global.get(&project_name),
            self.local.get(&project_name),
        ) {
            Ok(Project { global, local })
        } else {
            Err(Error::new(format!(
                "fail to found project {}",
                project_name.as_ref()
            )))
        }
    }

    pub fn set_temporary_current_project(&mut self, current_project: CurrentProject) {
        self.temp_current_project = Some(current_project);
    }

    pub fn current_project(&self) -> Result<Project> {
        let project_name = match &self.temp_current_project {
            Some(current_project) => current_project.name()?,
            None => self.global.current_project()?.name()?,
        };
        self.found(project_name)
    }

    pub fn current_env(&self) -> Result<String> {
        let env = match &self.temp_current_project {
            Some(current_project) => current_project.env()?,
            None => self.global.current_project()?.env()?,
        };
        Ok(env)
    }

    pub fn set_current_project_name<P: AsRef<str>>(&mut self, project_name: P) {
        self.global.set_current_project_name(project_name)
    }

    pub fn set_current_env_name<E: AsRef<str>>(&mut self, env: E) -> Result<()> {
        self.global.set_current_env_name(env)
    }

    pub fn save(&self) -> Result<()> {
        // TODO : save local too
        self.global.save()
    }

    pub fn fake() -> Projects {
        Projects {
            global: GlobalProjects::fake(),
            local: LocalProjects::fake(),
            temp_current_project: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::project::{CurrentProject, Projects};
    use std::path::PathBuf;

    #[test]
    fn current_template() {
        let projects = Projects::fake();
        let current_template_file = projects.current_project().unwrap().template_file().unwrap();
        assert_eq!(
            current_template_file,
            PathBuf::from("/path/to/local/project_test.tpl")
        );
    }

    #[test]
    fn current_project() {
        let projects = Projects::fake();
        let current_project = projects.current_project().unwrap();
        assert_eq!(current_project.name(), String::from("project_test"));
    }

    #[test]
    fn current_env() {
        let projects = Projects::fake();
        let current_env = projects.current_env().unwrap();
        assert_eq!(current_env, String::from("env_test"));
    }

    #[test]
    fn temporary_current_project() {
        let mut projects = Projects::fake();
        projects.set_temporary_current_project(CurrentProject::new("project_test_bis"));

        let current_project = projects.current_project().unwrap();
        assert_eq!(current_project.name(), String::from("project_test_bis"));
        assert!(projects.current_env().is_err());

        let mut projects = Projects::fake();
        projects.set_temporary_current_project(
            CurrentProject::new("project_test_bis").set_env("watever_env"),
        );
        assert_eq!(projects.current_env().unwrap(), String::from("watever_env"));
    }
}
