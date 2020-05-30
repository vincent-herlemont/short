use crate::cfg::global::setup::GlobalProjectSetupCfg;
use crate::cfg::SetupsCfg;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::path::{PathBuf};
use std::rc::Rc;

type SetupName = String;
type EnvName = String;

#[derive(Debug, Serialize, Deserialize)]
struct CurrentSetup {
    #[serde(rename = "setup", skip_serializing_if = "Option::is_none")]
    pub setup_name: Option<SetupName>,
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env_name: Option<EnvName>,
}

impl CurrentSetup {
    pub fn new() -> Self {
        Self {
            setup_name: None,
            env_name: None,
        }
    }
}

impl Default for CurrentSetup {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalProjectCfg {
    file: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    current: Option<CurrentSetup>,
    setups: Rc<RefCell<Vec<Rc<RefCell<GlobalProjectSetupCfg>>>>>,
}

impl GlobalProjectCfg {
    pub fn new(file: &PathBuf) -> Result<Self> {
        let mut gp = GlobalProjectCfg {
            file: PathBuf::new(),
            current: None,
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

    pub fn set_current_setup_name(&mut self, setup_name: SetupName) {
        self.current.get_or_insert(CurrentSetup::new()).setup_name = Some(setup_name);
    }

    pub fn current_setup_name(&self) -> Option<&SetupName> {
        self.current
            .as_ref()
            .map_or(None, |current| current.setup_name.as_ref())
    }

    pub fn set_current_env_name(&mut self, env_name: EnvName) {
        self.current.get_or_insert(CurrentSetup::new()).env_name = Some(env_name);
    }

    pub fn current_env_name(&self) -> Option<&EnvName> {
        self.current
            .as_ref()
            .map_or(None, |current| current.env_name.as_ref())
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
