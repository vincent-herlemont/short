use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

pub use setup::LocalSetupCfg;
pub use setup_provider::LocalSetupProviderCfg;
pub use setup_provider::LocalSetupProviderCloudformationCfg;

use crate::cfg::{EnvPathCfg, EnvPathsCfg};
use crate::cfg::new::NewCfg;
use crate::cfg::setup::{SetupCfg, SetupsCfg};

mod setup;
mod setup_provider;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalCfg {
    setups: Vec<Rc<RefCell<LocalSetupCfg>>>,
}

impl NewCfg for LocalCfg {
    type T = Self;
    fn new() -> Self {
        Self { setups: vec![] }
    }
}

impl SetupsCfg for LocalCfg {
    type Setup = LocalSetupCfg;

    fn add_setup(&mut self, setup: Self::Setup) {
        self.setups.append(&mut vec![Rc::new(RefCell::new(setup))]);
    }

    fn remove_by_name_setup(&mut self, name: String) {
        self.setups.retain(|setup| {
            let setup = setup.borrow();
            setup.name() != name
        });
    }

    fn get_setup(&self, name: String) -> Option<Rc<RefCell<Self::Setup>>> {
        self.setups
            .iter()
            .find(|setup| setup.borrow().name() == name)
            .map(|setup| Rc::clone(setup))
    }
}

impl EnvPathsCfg for LocalCfg {
    fn env_paths_dyn(&self) -> Vec<Rc<RefCell<dyn EnvPathCfg>>> {
        self.setups
            .iter()
            .map(|e| Rc::clone(e) as Rc<RefCell<dyn EnvPathCfg>>)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;

    use crate::cfg::{EnvPathCfg, EnvPathsCfg};
    use crate::cfg::{LocalCfg, LocalSetupCfg, LocalSetupProviderCfg};
    use crate::cfg::NewCfg;
    use crate::cfg::setup::SetupsCfg;

    #[test]
    fn local_update_public_env_directory() {
        let provider_cfg = LocalSetupProviderCfg::None;
        let setup_cfg_1 = LocalSetupCfg::new("setup_1".into(), provider_cfg);

        let mut local_cfg = LocalCfg::new();
        local_cfg.add_setup(setup_cfg_1);

        let env_path = local_cfg.env_paths();
        assert_eq!(env_path, vec![PathBuf::new()]);

        {
            let setup_cfg_1 = local_cfg.get_setup("setup_1".into()).unwrap();
            setup_cfg_1
                .borrow_mut()
                .set_env_path_op(Some("./env_dir/".into()));
        }

        let env_path = local_cfg.env_paths();
        assert_eq!(env_path, vec![PathBuf::from("./env_dir/")]);
    }
}
