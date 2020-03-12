use crate::project::global::GlobalProjects;
use crate::project::local::LocalProjects;
use std::path::Path;
use utils::error::Error;
use utils::result::Result;

pub mod global;
pub mod local;

#[allow(dead_code)]
pub struct Projects {
    local: LocalProjects,
    global: GlobalProjects,
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
            (Ok(local), Ok(global)) => Ok(Projects { local, global }),
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
        self.local
            .add(project_name, template_path, public_env_directory)?;
        Ok(())
    }
}
