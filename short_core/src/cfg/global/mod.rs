use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::cfg::global::project::GlobalProjectCfg;

mod project;
mod setup;

pub const GLOCAL_FILE_NAME: &'static str = "cfg.yml";

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalCfg {
    projects: Vec<GlobalProjectCfg>,
}

impl GlobalCfg {
    pub fn new() -> Self {
        Self { projects: vec![] }
    }

    pub fn add_project(&mut self, project: GlobalProjectCfg) {
        self.projects.append(&mut vec![project]);
    }
}

#[cfg(test)]
mod tests {
    use crate::cfg::global::project::GlobalProjectCfg;
    use crate::cfg::GlobalCfg;

    #[test]
    fn project() {
        let project_cfg = GlobalProjectCfg::new("/project".into()).unwrap();
        let mut global_cfg = GlobalCfg::new();
        global_cfg.add_project(project_cfg);
        dbg!(global_cfg);
    }
}
