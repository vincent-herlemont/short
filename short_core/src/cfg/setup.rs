use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::cfg::{GlobalCfg, LocalCfg};

pub trait SetupsCfg {
    type Setup;

    fn add_setup(&mut self, setup: Self::Setup);

    fn remove_by_name_setup(&mut self, name: String);

    fn get_setup(&self, name: String) -> Option<Rc<RefCell<Self::Setup>>>;
}

pub trait SetupCfg {
    fn name(&self) -> String;
}

#[derive(Debug)]
pub struct Setup {
    local_setup: Weak<RefCell<LocalCfg>>,
    global_setup: Weak<RefCell<GlobalCfg>>,
}

impl Setup {
    pub fn new() -> Self {
        Self {
            local_setup: Weak::default(),
            global_setup: Weak::default(),
        }
    }
}
