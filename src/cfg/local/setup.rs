use crate::cfg::local::ArrayVars;
use crate::cfg::setup::SetupCfg;
use crate::cfg::EnvPathCfg;
use serde::{Deserialize, Serialize};

use crate::cfg::local::setup_vars::Vars;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalSetupCfg {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    public_env_dir: Option<PathBuf>,

    file: PathBuf,

    #[serde(skip_serializing_if = "Option::is_none")]
    array_vars: Option<Rc<RefCell<ArrayVars>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vars: Option<Rc<RefCell<Vars>>>,
}

impl LocalSetupCfg {
    pub fn new(name: String, file: PathBuf) -> Self {
        let mut local_setup = Self {
            name,
            public_env_dir: None,
            file,
            array_vars: None,
            vars: None,
        };

        local_setup.new_array_vars();
        local_setup.new_vars();

        local_setup
    }

    pub fn new_vars(&mut self) -> Rc<RefCell<Vars>> {
        let mut vars = Vars::new();
        vars.add("SETUP_NAME".into());

        let vars = Rc::new(RefCell::new(vars));
        self.vars = Some(Rc::clone(&vars));
        vars
    }

    pub fn new_array_vars(&mut self) -> Rc<RefCell<ArrayVars>> {
        let mut array_vars = ArrayVars::new();
        array_vars.add("all".into(), ".*".into());

        let array_vars = Rc::new(RefCell::new(array_vars));
        self.array_vars = Some(Rc::clone(&array_vars));
        array_vars
    }

    pub fn array_vars(&self) -> Option<Rc<RefCell<ArrayVars>>> {
        self.array_vars.as_ref().map(|r| Rc::clone(r))
    }

    pub fn vars(&self) -> Option<Rc<RefCell<Vars>>> {
        self.vars.as_ref().map(|r| Rc::clone(r))
    }
}

impl SetupCfg for LocalSetupCfg {
    fn name(&self) -> &String {
        &self.name
    }

    fn rename(&mut self, name: &String) {
        self.name = name.clone();
    }
}

impl EnvPathCfg for LocalSetupCfg {
    fn env_path_op(&self) -> Option<&PathBuf> {
        self.public_env_dir.as_ref()
    }

    fn set_env_path_op(&mut self, dir: Option<PathBuf>) {
        self.public_env_dir = dir
    }
}
