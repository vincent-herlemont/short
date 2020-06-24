use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

pub use setup::LocalSetupCfg;
pub use setup_array_vars::{ArrayVar, ArrayVars};
pub use setup_vars::{Var, Vars};

use crate::cfg::setup::SetupsCfg;

mod setup;
mod setup_array_vars;
mod setup_vars;

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

#[cfg(test)]
mod tests {
    use crate::cfg::{LocalCfg, LocalSetupCfg};
    use crate::cfg::setup::SetupsCfg;

    #[test]
    fn local_update_public_env_dir() {
        let setup_cfg = LocalSetupCfg::new("setup".into(), "run.sh".into());

        let mut local_cfg = LocalCfg::new();
        local_cfg.add_setup(setup_cfg);

        {
            let setup_cfg_1 = local_cfg.get_setup(&"setup".into()).unwrap();
            let mut setup_cfg_1 = setup_cfg_1.borrow_mut();
            setup_cfg_1.set_public_env_dir("./env_dir/".into());
        }

        local_cfg.remove_by_name_setup(&"setup".into());
        assert!(local_cfg.get_setup(&"setup".into()).is_none());
    }

    #[test]
    fn local_cfg_yaml() {
        let setup_cfg = LocalSetupCfg::new("setup".into(), "run.sh".into());

        let expect = r#"---
name: setup
file: run.sh
array_vars:
  all: ".*"
  var2: SUFFIX_*
  var1: PREFIX_*
vars:
  - SETUP_NAME"#;

        let array_vars = setup_cfg.array_vars().unwrap();
        let mut array_vars = array_vars.borrow_mut();
        array_vars.add("all".into(), ".*".into());
        array_vars.add("var2".into(), "SUFFIX_*".into());
        array_vars.add("var1".into(), "PREFIX_*".into());
        drop(array_vars);

        let content = serde_yaml::to_string(&setup_cfg).unwrap();
        assert_eq!(expect, content.as_str());

        let setup_cfg: LocalSetupCfg = serde_yaml::from_str(content.as_str()).unwrap();
        let content = serde_yaml::to_string(&setup_cfg).unwrap();
        assert_eq!(expect, content.as_str());
    }
}
