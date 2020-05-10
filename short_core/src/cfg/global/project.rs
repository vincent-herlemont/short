use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::cfg::global::setup::GlobalProjectSetupCfg;
use crate::cfg::{ProjectCfg, SetupCfg, SetupsCfg};

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalProjectCfg {
    file: PathBuf,
    setups: Rc<RefCell<Vec<Rc<RefCell<GlobalProjectSetupCfg>>>>>,
}

impl GlobalProjectCfg {
    pub fn new(file: &PathBuf) -> Result<Self> {
        let mut gp = GlobalProjectCfg {
            file: PathBuf::new(),
            setups: Rc::new(RefCell::new(vec![])),
        };
        gp.set_file(file)?;
        Ok(gp)
    }

    pub fn set_file(&mut self, file: &PathBuf) -> Result<()> {
        if (!file.is_absolute()) {
            return Err(anyhow!(format!(
                "project file path can not be relative {}",
                file.to_string_lossy()
            )));
        }
        if let None = file.file_name() {
            return Err(anyhow!(format!("project file has no name")));
        }
        self.file = file.clone();
        Ok(())
    }
}

impl ProjectCfg for GlobalProjectCfg {
    fn path(&self) -> PathBuf {
        self.file.to_owned()
    }
}

impl SetupsCfg for GlobalProjectCfg {
    type Setup = GlobalProjectSetupCfg;

    fn get_setups(&self) -> Rc<RefCell<Vec<Rc<RefCell<Self::Setup>>>>> {
        Rc::clone(&self.setups)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::cfg::global::project::GlobalProjectCfg;
    use crate::cfg::global::setup::GlobalProjectSetupCfg;
    use crate::cfg::{EnvPathCfg, SetupsCfg};

    #[test]
    fn global_update_private_env_dir() {
        let setup_cfg = GlobalProjectSetupCfg::new("setup".into());

        let mut project_cfg = GlobalProjectCfg::new(&"/project".into()).unwrap();
        project_cfg.add_setup(setup_cfg);

        assert!(project_cfg.get_setups().borrow().iter().count().eq(&1));

        {
            let setup_cfg = project_cfg.get_setup("setup".into()).unwrap();
            setup_cfg
                .borrow_mut()
                .set_env_path_op(Some("/private_env".into()));
        }

        let global_project_setup_cfg_1 = project_cfg.get_setup("setup".into()).unwrap();
        assert_eq!(
            global_project_setup_cfg_1.borrow().env_path(),
            PathBuf::from("/private_env")
        );

        project_cfg.remove_by_name_setup("setup".into());
        assert!(project_cfg.get_setup("setup".into()).is_none());
    }
}
