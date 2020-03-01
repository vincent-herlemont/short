use std::fs::create_dir;
use std::path::Path;
use utils::error::Error;
use utils::result::Result;

#[allow(dead_code)]
const PROJECT_CONFIG_DIRECTORY: &'static str = ".d4d";

#[allow(dead_code)]
fn create_project_config_directory<P: AsRef<Path>>(home: P) -> Result<()> {
    let project_config_directory = home.as_ref().join(PROJECT_CONFIG_DIRECTORY);
    match create_dir(project_config_directory) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::from(e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::projet::create_project_config_directory;
    use insta::assert_debug_snapshot;
    use std::collections::HashMap;
    use utils::test::before;
    use walkdir::WalkDir;

    #[test]
    fn test_create_project_config_directory() {
        let config = before("create_project_config_directory", HashMap::new());
        let r = create_project_config_directory(&config.tmp_dir);
        assert_debug_snapshot!(r);
        let r: Vec<_> = WalkDir::new(&config.tmp_dir).into_iter().collect();
        assert_debug_snapshot!(r.iter().count());
        let r = create_project_config_directory(&config.tmp_dir);
        assert_debug_snapshot!(r);
    }
}
