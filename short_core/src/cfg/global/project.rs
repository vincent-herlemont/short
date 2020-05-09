use crate::cfg::global::setup::GlobalProjectSetupCfg;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalProjectCfg {
    path: PathBuf,
    setups: Vec<Rc<RefCell<GlobalProjectSetupCfg>>>,
}
