use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

use crate::cfg::{GlobalCfg, LocalCfg};

pub trait SetupsCfg {
    type Setup: SetupCfg;

    fn add_setup(&mut self, setup: Self::Setup) {
        if let None = self.get_setup(setup.name()) {
            self.get_setups()
                .borrow_mut()
                .append(&mut vec![Rc::new(RefCell::new(setup))])
        }
    }

    fn remove_by_name_setup(&mut self, name: String) {
        self.get_setups().borrow_mut().retain(|setup| {
            let setup = setup.borrow();
            setup.name() != name
        });
    }

    fn get_setup(&self, name: String) -> Option<Rc<RefCell<Self::Setup>>> {
        self.get_setups()
            .borrow()
            .iter()
            .find(|setup| setup.borrow().name() == name)
            .map(|setup| Rc::clone(setup))
    }

    fn get_setups(&self) -> Rc<RefCell<Vec<Rc<RefCell<Self::Setup>>>>>;
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
