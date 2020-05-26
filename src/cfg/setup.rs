use std::cell::RefCell;
use std::fmt;

use crate::env_file;
use crate::env_file::Env;
use anyhow::Context;
use anyhow::Result;
use serde::export::fmt::Debug;
use serde::export::Formatter;
use std::path::PathBuf;
use std::rc::{Rc, Weak};

use crate::cfg::{EnvPathCfg, LocalSetupCfg};

use crate::cfg::global::GlobalProjectSetupCfg;

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

#[derive(Clone)]
pub struct Setup {
    local_cfg_file: Option<PathBuf>,
    local_setup: Weak<RefCell<LocalSetupCfg>>,
    global_setup: Weak<RefCell<GlobalProjectSetupCfg>>,
}

impl Setup {
    pub fn new() -> Self {
        Self {
            local_cfg_file: None,
            local_setup: Weak::default(),
            global_setup: Weak::default(),
        }
    }

    pub fn envs(&self) -> Vec<Result<Env>> {
        let mut env = vec![];
        env.append(&mut self.envs_public());
        env.append(&mut self.envs_private());
        env
    }

    pub fn envs_public(&self) -> Vec<Result<Env>> {
        if let (Some(local_setup), Some(file)) = (&self.local_setup(), &self.local_cfg_file) {
            if let Some(dir) = file.parent() {
                let abs_path = dir.join(local_setup.borrow().env_path());
                let env = env_file::read_dir(&abs_path);
                return env
                    .into_iter()
                    .map(|env| env.context("fail to parse env"))
                    .collect();
            }
        }
        vec![]
    }

    pub fn envs_private(&self) -> Vec<Result<Env>> {
        if let Some(global_setup) = self.global_setup() {
            let env = env_file::read_dir(&global_setup.borrow().env_path());
            return env
                .into_iter()
                .map(|env| env.context("fail to parse env"))
                .collect();
        }
        vec![]
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
        local_file: &PathBuf,
        local_setup: Weak<RefCell<LocalSetupCfg>>,
        global_setup: Weak<RefCell<GlobalProjectSetupCfg>>,
    ) -> Result<Self> {
        let rc_local_setup = local_setup.upgrade().context("local set up cfg is empty")?;
        let rc_global_setup = global_setup
            .upgrade()
            .context("global set up cfg is empty")?;
        if rc_local_setup.borrow().name() == rc_global_setup.borrow().name() {
            Ok(Self {
                local_cfg_file: Some(local_file.to_owned()),
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
