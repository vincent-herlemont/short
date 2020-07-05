use std::borrow::Cow;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::cfg::local::setup_vars::Vars;
use crate::cfg::local::ArrayVars;
use crate::cfg::setup::SetupCfg;
use crate::cfg::{ArrayVar, CfgError};

pub type SetupName = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalSetupCfg {
    #[serde(skip)]
    name: SetupName,

    #[serde(skip_serializing_if = "Option::is_none")]
    public_env_dir: Option<PathBuf>,

    file: PathBuf,

    #[serde(skip_serializing_if = "Option::is_none")]
    array_vars: Option<Rc<RefCell<ArrayVars>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vars: Option<Rc<RefCell<Vars>>>,
}

impl Clone for LocalSetupCfg {
    fn clone(&self) -> Self {
        let array_vars = self.array_vars.as_ref().map(|array_vars| {
            let array_vars = Rc::clone(array_vars);
            let array_vars = (&*array_vars).clone();
            Rc::new(array_vars)
        });

        let vars = self.vars.as_ref().map(|vars| {
            let vars = Rc::clone(vars);
            let vars = (&*vars).clone();
            Rc::new(vars)
        });

        Self {
            name: self.name.clone(),
            public_env_dir: self.public_env_dir.clone(),
            file: self.file.clone(),
            array_vars,
            vars,
        }
    }
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

    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn set_file(&mut self, file: PathBuf) {
        self.file = file;
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
        array_vars.add(ArrayVar::new("all".into(), ".*".into()));

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

    pub fn public_env_dir(&self) -> Cow<Path> {
        match &self.public_env_dir {
            Some(dir) => Cow::Borrowed(dir),
            None => Cow::Owned(PathBuf::new()),
        }
    }

    pub fn set_public_env_dir(&mut self, dir: PathBuf) {
        self.public_env_dir = Some(dir)
    }

    pub fn unset_public_env_dir(&mut self) -> Result<()> {
        if let None = self.public_env_dir {
            bail!(CfgError::PublicEnvAlreadyUnset(self.name.clone()))
        } else {
            self.public_env_dir = None;
            Ok(())
        }
    }
}

impl SetupCfg for LocalSetupCfg {
    fn name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, name: SetupName) {
        self.name = name;
    }
}

#[cfg(test)]
mod tests {
    use crate::cfg::{ArrayVar, LocalSetupCfg};

    #[test]
    fn local_cfg_yaml() {
        let setup_cfg = LocalSetupCfg::new("setup".into(), "run.sh".into());

        let expect = r#"---
file: run.sh
array_vars:
  all: ".*"
  var2: "*_SUFFIX"
  var1: PREFIX_*
vars:
  - SETUP_NAME"#;

        let array_vars = setup_cfg.array_vars().unwrap();
        let mut array_vars = array_vars.borrow_mut();
        array_vars.add(ArrayVar::new("all".into(), ".*".into()));
        array_vars.add(ArrayVar::new("var2".into(), "*_SUFFIX".into()));
        array_vars.add(ArrayVar::new("var1".into(), "PREFIX_*".into()));
        drop(array_vars);

        let content = serde_yaml::to_string(&setup_cfg).unwrap();
        assert_eq!(expect, content.as_str());

        let setup_cfg: LocalSetupCfg = serde_yaml::from_str(content.as_str()).unwrap();
        let content = serde_yaml::to_string(&setup_cfg).unwrap();
        assert_eq!(expect, content.as_str());
    }
}
