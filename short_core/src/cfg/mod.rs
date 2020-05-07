mod env;
mod file;
mod global;
mod local;
mod setup;

pub use env::EnvPathCfg;
pub use env::EnvPathsCfg;

pub use local::LocalCfg;
pub use local::LocalSetupCfg;
pub use local::LocalSetupProviderCfg;

pub use global::GlobalCfg;
pub use global::GlobalProjectsCfg;

use crate::cfg::file::FileCfg;
use crate::cfg::setup::Setup;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Cfg {
    current: Option<Rc<RefCell<dyn Setup>>>,
    local: FileCfg<LocalCfg>,
    global: FileCfg<GlobalCfg>,
}

#[cfg(test)]
mod test {
    use short_utils::integration_test::environment::IntegrationTestEnvironment;

    const HOME: &'static str = "home";
    const PROJECT: &'static str = "project";

    fn init_env() -> IntegrationTestEnvironment {
        let mut e = IntegrationTestEnvironment::new("cfg");
        e.add_dir(HOME);
        e.add_dir(PROJECT);
        e.setup();
        e
    }

    #[test]
    fn load_cfg() {
        let e = init_env();
    }
}
