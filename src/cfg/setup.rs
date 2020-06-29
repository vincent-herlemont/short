use std::cell::RefCell;
use std::fmt;
use std::path::PathBuf;
use std::rc::{Rc, Weak};

use anyhow::{Context, Result};
use serde::export::fmt::Debug;
use serde::export::Formatter;

use crate::cfg::error::CfgError;
use crate::cfg::global::GlobalProjectSetupCfg;
use crate::cfg::LocalSetupCfg;
use crate::env_file;
use crate::env_file::{path_from_env_name, Env};

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

    pub fn local_cfg_file(&self) -> Result<&PathBuf> {
        self.local_cfg_file.as_ref().context("no local cfg file")
    }

    pub fn local_cfg_dir(&self) -> Result<PathBuf> {
        let local_cfg_file = self.local_cfg_file()?;
        let local_cfg_dir = local_cfg_file
            .parent()
            .context(format!("can not reach parent of {:?}", local_cfg_file))?;
        Ok(local_cfg_dir.to_path_buf())
    }

    pub fn local_cfg_run_file(&self) -> Result<PathBuf> {
        let local_cfg_dir = self.local_cfg_dir()?;
        let local_setup = self.local_setup().context("local_setup not found")?;
        let local_setup = local_setup.borrow();
        let run_file = local_cfg_dir.join(local_setup.file());
        Ok(run_file)
    }

    pub fn env(&self, env_name: &String) -> Result<Env> {
        let env_file = self.env_file(env_name)?;
        let env = Env::from_file_reader(env_file)?;
        Ok(env)
    }

    pub fn env_file(&self, env_name: &String) -> Result<PathBuf> {
        match (self.envs_private_dir(), self.envs_public_dir()) {
            (Ok(private_dir), Ok(public_dir)) => {
                let public_env = path_from_env_name(&public_dir, env_name);
                let private_env = path_from_env_name(&private_dir, env_name);
                if private_env.exists() && public_env.exists() {
                    Err(CfgError::EnvExistTwice(env_name.clone(), public_env, private_env).into())
                } else if private_env.exists() {
                    Ok(private_env)
                } else if public_env.exists() {
                    Ok(public_env)
                } else {
                    Err(CfgError::EnvNotFound(env_name.clone()).into())
                }
            }
            (Ok(private_dir), Err(_)) => {
                let private_env = path_from_env_name(&private_dir, env_name);
                if private_env.exists() {
                    Ok(private_env)
                } else {
                    Err(CfgError::EnvNotFound(env_name.clone()).into())
                }
            }
            (Err(_), Ok(public_env)) => {
                let public_env = path_from_env_name(&public_env, env_name);
                if public_env.exists() {
                    Ok(public_env)
                } else {
                    Err(CfgError::EnvNotFound(env_name.clone()).into())
                }
            }
            (_, Err(err)) => Err(err),
        }
    }

    pub fn envs(&self) -> Vec<Result<Env>> {
        let mut env = vec![];
        env.append(&mut self.envs_public());
        env.append(&mut self.envs_private());
        env
    }

    pub fn envs_public_dir(&self) -> Result<PathBuf> {
        if let (Some(local_setup), Some(file)) = (&self.local_setup(), &self.local_cfg_file) {
            if let Some(root_dir) = file.parent() {
                let local_setup = local_setup.borrow();
                return Ok(root_dir.join(local_setup.public_env_dir()));
            }
        }
        bail!(CfgError::PublicEnvDirNotFound(self.name()?))
    }

    pub fn envs_public(&self) -> Vec<Result<Env>> {
        if let Ok(abs_path) = self.envs_public_dir() {
            let env = env_file::read_dir(&abs_path);
            return env
                .into_iter()
                .map(|env| env.context("fail to parse public env"))
                .collect();
        }
        vec![]
    }

    pub fn envs_private_dir(&self) -> Result<PathBuf> {
        if let Some(global_setup) = self.global_setup() {
            if let Ok(dir) = global_setup.borrow().private_env_dir() {
                return Ok(dir.clone());
            }
        }
        bail!(CfgError::PrivateEnvDirNotFound(self.name()?))
    }

    pub fn envs_private(&self) -> Vec<Result<Env>> {
        if let Ok(global_setup) = self.envs_private_dir() {
            let env = env_file::read_dir(&global_setup);
            return env
                .into_iter()
                .map(|env| env.context("fail to parse private env"))
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
        local_setup: &Rc<RefCell<LocalSetupCfg>>,
        global_setup: &Rc<RefCell<GlobalProjectSetupCfg>>,
    ) -> Result<Self> {
        if local_setup.borrow().name() == global_setup.borrow().name() {
            Ok(Self {
                local_cfg_file: Some(local_file.to_owned()),
                local_setup: Rc::downgrade(local_setup),
                global_setup: Rc::downgrade(global_setup),
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
