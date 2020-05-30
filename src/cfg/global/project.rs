use crate::cfg::global::setup::GlobalProjectSetupCfg;
use crate::cfg::SetupsCfg;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

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
        if !file.is_absolute() {
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

    pub fn file(&self) -> &PathBuf {
        &self.file
    }
}

impl SetupsCfg for GlobalProjectCfg {
    type Setup = GlobalProjectSetupCfg;

    fn get_setups(&self) -> Rc<RefCell<Vec<Rc<RefCell<Self::Setup>>>>> {
        Rc::clone(&self.setups)
    }
}

impl PartialEq<PathBuf> for GlobalProjectCfg {
    fn eq(&self, path_buf: &PathBuf) -> bool {
        self.file().eq(path_buf)
    }
}
impl PartialEq<GlobalProjectCfg> for PathBuf {
    fn eq(&self, path_buf: &GlobalProjectCfg) -> bool {
        self.eq(&path_buf.file)
    }
}

#[cfg(test)]
mod test {
    use crate::cfg::global::project::GlobalProjectCfg;
    use crate::cfg::global::setup::GlobalProjectSetupCfg;
    use crate::cfg::SetupsCfg;
    use std::path::PathBuf;

    #[test]
    fn global_update_private_env_dir() {
        let setup_cfg = GlobalProjectSetupCfg::new("setup".into());

        let mut project_cfg = GlobalProjectCfg::new(&"/project".into()).unwrap();
        project_cfg.add_setup(setup_cfg);

        assert!(project_cfg.get_setups().borrow().iter().count().eq(&1));

        {
            let setup_cfg = project_cfg.get_setup(&"setup".into()).unwrap();
            setup_cfg
                .borrow_mut()
                .set_private_env_dir("/private_env".into());
        }

        let global_project_setup_cfg_1 = project_cfg.get_setup(&"setup".into()).unwrap();
        assert_eq!(
            global_project_setup_cfg_1
                .borrow()
                .private_env_dir()
                .unwrap(),
            &PathBuf::from("/private_env")
        );

        project_cfg.remove_by_name_setup(&"setup".into());
        assert!(project_cfg.get_setup(&"setup".into()).is_none());
    }
}
