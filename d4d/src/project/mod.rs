use crate::project::global::GlobalProjects;
use crate::project::local::LocalProjects;
use std::path::Path;
use utils::error::Error;
use utils::result::Result;

pub mod global;
pub mod local;

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
}
