use anyhow::{Context, Result};
use short_env::Env;
use std::cell::RefCell;
use std::fs::read_dir;
use std::path::PathBuf;
use std::rc::Rc;

pub trait EnvPathCfg {
    fn env_path(&self) -> PathBuf {
        self.env_path_op()
            .map_or(PathBuf::new(), |path_buf| path_buf.clone())
    }

    fn env_path_op(&self) -> Option<&PathBuf>;

    fn set_env_path_op(&mut self, dir: Option<PathBuf>);
}

pub trait EnvPathsCfg {
    fn env_paths(&self) -> Vec<PathBuf> {
        self.env_paths_dyn()
            .iter()
            .map(|env_path| env_path.borrow().env_path())
            .collect()
    }

    fn env_paths_dyn(&self) -> Vec<Rc<RefCell<dyn EnvPathCfg>>>;
}
