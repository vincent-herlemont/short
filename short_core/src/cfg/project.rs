use std::path::PathBuf;

pub trait ProjectCfg {
    fn path(&self) -> PathBuf;
}
