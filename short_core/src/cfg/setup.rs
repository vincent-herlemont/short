use crate::cfg::global::GlobalProjectSetupCfg;
use crate::cfg::{GlobalCfg, LocalCfg, LocalSetupCfg};
use anyhow::Context;
use anyhow::Result;
use serde::export::fmt::Debug;
use serde::export::Formatter;
use std::cell::{Ref, RefCell};
use std::fmt;
use std::fmt::Display;
use std::rc::{Rc, Weak};

pub trait SetupsCfg {
    type Setup: SetupCfg;

    fn add_setup(&mut self, setup: Self::Setup) {
        if let None = self.get_setup(setup.name()) {
            self.get_setups()
                .borrow_mut()
                .append(&mut vec![Rc::new(RefCell::new(setup))])
        }
    }

    fn remove_by_name_setup(&mut self, name: &String) {
        self.get_setups().borrow_mut().retain(|setup| {
            let setup = setup.borrow();
            setup.name() != name
        });
    }

    fn get_setup(&self, name: &String) -> Option<Rc<RefCell<Self::Setup>>> {
        self.get_setups()
            .borrow()
            .iter()
            .find(|setup| setup.borrow().name() == name)
            .map(|setup| Rc::clone(setup))
    }

    fn get_setups(&self) -> Rc<RefCell<Vec<Rc<RefCell<Self::Setup>>>>>;
}

pub trait SetupCfg {
    fn name(&self) -> &String;

    fn rename(&mut self, name: &String);
}

pub struct Setup {
    local_setup: Weak<RefCell<LocalSetupCfg>>,
    global_setup: Weak<RefCell<GlobalProjectSetupCfg>>,
}

impl Setup {
    pub fn new() -> Self {
        Self {
            local_setup: Weak::default(),
            global_setup: Weak::default(),
        }
    }

    pub fn name(&self) -> Result<String> {
        if let Some(local_setup) = self.local_setup() {
            return Ok(local_setup.borrow().name().clone());
        }
        if let Some(global_setup) = self.global_setup() {
            return Ok(global_setup.borrow().name().clone());
        }
        Err(anyhow!(
            "fail to get name : local and global cfg are not sets"
        ))
    }

    pub fn rename(&self, name: &String) -> Result<()> {
        let mut bool = false;
        if let Some(local_setup) = self.local_setup() {
            local_setup.borrow_mut().rename(name);
            bool = true;
        }
        if let Some(global_setup) = self.global_setup() {
            global_setup.borrow_mut().rename(name);
            bool = true;
        }
        if !bool {
            return Err(anyhow!(
                "fail to rename : local and global cfg are not sets"
            ));
        }
        Ok(())
    }

    pub fn new_fill(
        local_setup: Weak<RefCell<LocalSetupCfg>>,
        global_setup: Weak<RefCell<GlobalProjectSetupCfg>>,
    ) -> Result<Self> {
        let rc_local_setup = local_setup.upgrade().context("local set up cfg is empty")?;
        let rc_global_setup = global_setup
            .upgrade()
            .context("global set up cfg is empty")?;
        if rc_local_setup.borrow().name() == rc_global_setup.borrow().name() {
            Ok(Self {
                local_setup,
                global_setup,
            })
        } else {
            Err(anyhow!(
                "local setup and global setup must has the same name"
            ))
        }
    }

    pub fn local_setup(&self) -> Option<Rc<RefCell<LocalSetupCfg>>> {
        self.local_setup.upgrade()
    }

    pub fn global_setup(&self) -> Option<Rc<RefCell<GlobalProjectSetupCfg>>> {
        self.global_setup.upgrade()
    }
}

impl Debug for Setup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "local : {:#?}\n", self.local_setup())?;
        write!(f, "global : {:#?}\n", self.global_setup())?;
        Ok(())
    }
}
