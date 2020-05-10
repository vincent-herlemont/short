use std::path::PathBuf;

pub trait ProjectCfg {
    fn path(&self) -> PathBuf;
}

impl PartialEq<PathBuf> for dyn ProjectCfg {
    fn eq(&self, other: &PathBuf) -> bool {
        self.path() == *other
    }
}

impl PartialEq<dyn ProjectCfg> for PathBuf {
    fn eq(&self, other: &dyn ProjectCfg) -> bool {
        other.path() == *self
    }
}
