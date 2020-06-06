use std::fs;
use std::path::PathBuf;

use crate::env_file::Env;
use crate::env_file::Result;

pub fn sync(envs: &Vec<Env>) {}

#[cfg(test)]
mod tests {
    use crate::env_file::Env;
    use fs_extra::dir::DirEntryAttr::Path;
    use std::path::PathBuf;

    #[test]
    fn test_sync() {
        let mut e1 = Env::new(PathBuf::new());
        e1.add("var1", "value1");
        let mut e2 = e1.copy(PathBuf::new());
        e2.add("var2", "value2");

        let v = vec![e1, e2];
    }
}
