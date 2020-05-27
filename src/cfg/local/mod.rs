mod setup;
mod setup_env_group;

use crate::cfg::setup::SetupsCfg;
use crate::cfg::{EnvPathCfg, EnvPathsCfg};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

pub use setup::LocalSetupCfg;
pub use setup_env_group::{EnvGroup, EnvGroups};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalCfg {
    setups: Rc<RefCell<Vec<Rc<RefCell<LocalSetupCfg>>>>>,
}

impl LocalCfg {
    pub fn new() -> Self {
        Self {
            setups: Rc::new(RefCell::new(vec![])),
        }
    }
}

impl SetupsCfg for LocalCfg {
    type Setup = LocalSetupCfg;

    fn get_setups(&self) -> Rc<RefCell<Vec<Rc<RefCell<Self::Setup>>>>> {
        Rc::clone(&self.setups)
    }
}

impl EnvPathsCfg for LocalCfg {
    fn env_paths_dyn(&self) -> Vec<Rc<RefCell<dyn EnvPathCfg>>> {
        self.setups
            .borrow()
            .iter()
            .map(|e| Rc::clone(e) as Rc<RefCell<dyn EnvPathCfg>>)
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use crate::cfg::setup::SetupsCfg;
    use crate::cfg::{EnvPathCfg, EnvPathsCfg};
    use crate::cfg::{LocalCfg, LocalSetupCfg};
    
    use std::path::PathBuf;

    #[test]
    fn local_update_public_env_dir() {
        let setup_cfg = LocalSetupCfg::new("setup".into(), "run.sh".into());

        let mut local_cfg = LocalCfg::new();
        local_cfg.add_setup(setup_cfg);

        let env_path = local_cfg.env_paths();
        assert_eq!(env_path, vec![PathBuf::new()]);

        {
            let setup_cfg_1 = local_cfg.get_setup(&"setup".into()).unwrap();
            let mut setup_cfg_1 = setup_cfg_1.borrow_mut();
            setup_cfg_1.set_env_path_op(Some("./env_dir/".into()));
        }

        let env_path = local_cfg.env_paths();
        assert_eq!(env_path, vec![PathBuf::from("./env_dir/")]);

        local_cfg.remove_by_name_setup(&"setup".into());
        assert!(local_cfg.get_setup(&"setup".into()).is_none());
    }

    #[test]
    fn local_cfg_yaml() {
        let setup_cfg = LocalSetupCfg::new("setup".into(), "run.sh".into());

        let expect = r#"---
name: setup
file: run.sh
env_groups:
  all: "*"
  var2: SUFFIX_*
  var1: PREFIX_*"#;

        let mut env_groups = setup_cfg.env_groups();
        env_groups.add("all".into(), "*".into());
        env_groups.add("var2".into(), "SUFFIX_*".into());
        env_groups.add("var1".into(), "PREFIX_*".into());

        let content = serde_yaml::to_string(&setup_cfg).unwrap();
        assert_eq!(expect, content.as_str());

        let setup_cfg: LocalSetupCfg = serde_yaml::from_str(content.as_str()).unwrap();
        let content = serde_yaml::to_string(&setup_cfg).unwrap();
        assert_eq!(expect, content.as_str());
    }
}