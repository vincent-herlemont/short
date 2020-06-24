use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn find_in_parents(dir: PathBuf, file_name: String) -> Result<PathBuf> {
    let file_path = dir.join(&file_name);
    if file_path.exists() {
        Ok(file_path)
    } else {
        let parent_path = dir
            .parent()
            .context("root directory reached")?
            .to_path_buf();
        find_in_parents(parent_path, file_name)
    }
}

#[cfg(test)]
mod test {
    use crate::utils::find::find_in_parents;
    use cli_integration_test::IntegrationTestEnvironment;
    use predicates::prelude::Predicate;
    use predicates::str::contains;
    use std::fs::read_to_string;
    use std::path::Path;

    #[test]
    fn find_local_cfg_file_on_root() {
        let mut e = IntegrationTestEnvironment::new("find_local_cfg");
        e.add_file("file1", "test");
        e.setup();

        find_in_parents(e.path().to_path_buf(), "file2".to_string()).unwrap_err();
        let path = find_in_parents(e.path().to_path_buf(), "file1".to_string()).unwrap();
        let str = read_to_string(path).unwrap();
        contains("test").eval(&str);
    }

    #[test]
    fn find_local_cfg_file_on_child() {
        let mut e = IntegrationTestEnvironment::new("find_local_cfg");
        let child_dirs = Path::new("dir1/dir2/dir3");
        e.add_dir(child_dirs);
        e.add_file("file1", "test");
        e.setup();

        let child_dirs = e.path().join(child_dirs);
        let path = find_in_parents(child_dirs.clone(), "file1".to_string()).unwrap();
        let str = read_to_string(path).unwrap();
        contains("test").eval(&str);
    }
}
