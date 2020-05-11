mod project;
mod setup;

use crate::cfg::file::FileCfg;
use crate::cfg::project::ProjectCfg;
use crate::cfg::{LocalCfg, LocalSetupCfg, SetupsCfg};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub use project::GlobalProjectCfg;
pub use setup::GlobalProjectSetupCfg;

pub const GLOCAL_FILE_NAME: &'static str = "cfg.yml";

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalCfg {
    projects: Vec<Rc<RefCell<GlobalProjectCfg>>>,
}

impl GlobalCfg {
    pub fn new() -> Self {
        Self { projects: vec![] }
    }

    pub fn add_project(&mut self, project: GlobalProjectCfg) {
        if let None = self.get_project_by_file(&project.path()) {
            self.projects
                .append(&mut vec![Rc::new(RefCell::new(project))]);
        }
    }

    pub fn remove_project_by_file(&mut self, file: &PathBuf) {
        self.projects.retain(|project| {
            let project = &*project.borrow() as &dyn ProjectCfg;
            file != project
        })
    }

    pub fn get_project_by_file(&self, file: &PathBuf) -> Option<Rc<RefCell<GlobalProjectCfg>>> {
        self.projects
            .iter()
            .find(|project| {
                let project = &*project.borrow() as &dyn ProjectCfg;
                file == project
            })
            .map(|project| Rc::clone(project))
    }

    pub fn sync_local_project(
        &mut self,
        file_local_cfg: &FileCfg<LocalCfg>,
    ) -> Result<Rc<RefCell<GlobalProjectCfg>>> {
        if let Ok(local_path) = file_local_cfg.file() {
            // Upsert global project
            let global_project = if let Some(global_project) = self.get_project_by_file(local_path)
            {
                global_project
            } else {
                let global_project = GlobalProjectCfg::new(local_path)?;
                self.add_project(global_project);
                self.get_project_by_file(local_path).unwrap()
            };

            // Sync local setup to global setup
            let local_setups = file_local_cfg.borrow().get_setups();
            for local_setup in local_setups.borrow().iter() {
                let global_setup = GlobalProjectSetupCfg::from(&*local_setup.borrow());
                global_project.borrow_mut().add_setup(global_setup)
            }
            Ok(global_project)
        } else {
            Err(anyhow!(format!(
                "file local cfg has no path {:?}",
                file_local_cfg
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cfg::global::project::GlobalProjectCfg;
    use crate::cfg::GlobalCfg;
    use std::path::PathBuf;

    #[test]
    fn project() {
        let path: PathBuf = "/project/short.yml".into();
        let project_cfg = GlobalProjectCfg::new(&path).unwrap();
        let mut global_cfg = GlobalCfg::new();

        // Add project to global conf
        global_cfg.add_project(project_cfg);
        global_cfg.add_project(GlobalProjectCfg::new(&path).unwrap()); // Ensure to remove duplicate project
        assert!(global_cfg.projects.iter().count().eq(&1));

        let change_path: PathBuf = "/project_1/short.yml".into();
        {
            // Get project and update change the file
            let project_cfg = global_cfg.get_project_by_file(&path).unwrap();
            project_cfg.borrow_mut().set_file(&change_path).unwrap();
        }

        // Try to delete first path : nothing append
        global_cfg.remove_project_by_file(&path);
        assert!(global_cfg.projects.iter().count().eq(&1));

        // Try to delete last path : delete append
        global_cfg.remove_project_by_file(&change_path);
        assert!(global_cfg.projects.iter().count().eq(&0));
    }
}
