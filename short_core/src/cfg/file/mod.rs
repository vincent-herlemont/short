use anyhow::{Context, Result};

use crate::cfg::file::find::find_local_cfg;
use crate::cfg::{GlobalCfg, LocalCfg};
use fs_extra::file::read_to_string;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

mod find;

#[derive(Debug)]
pub struct FileCfg<C>
where
    C: Serialize + DeserializeOwned,
{
    path: Option<PathBuf>,
    cfg: C,
}

impl<C> FileCfg<C>
where
    C: Serialize + DeserializeOwned,
{
    pub fn from_file<P>(path: P) -> Result<FileCfg<C>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let str = read_to_string(path)?;
        let cfg = serde_yaml::from_str(str.as_str())
            .context(format!("fail to parse {}", path.to_string_lossy()))?;
        Ok(Self {
            cfg,
            path: Some(path.to_path_buf()),
        })
    }

    pub fn new<P>(path: P) -> Result<FileCfg<C>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
    }

    pub fn from_local_file<P, F>(dir: P, file_name: F) -> Result<FileCfg<LocalCfg>>
    where
        P: AsRef<Path>,
        F: AsRef<str>,
    {
        let path = find_local_cfg(dir.as_ref().into(), file_name.as_ref().into())?;
        FileCfg::from_file(path)
    }

    pub fn from_global_file<P, F>(dir: P, file_name: F) -> Result<FileCfg<GlobalCfg>>
    where
        P: AsRef<Path>,
        F: AsRef<str>,
    {
        unimplemented!()
    }
}

impl From<LocalCfg> for FileCfg<LocalCfg> {
    fn from(cfg: LocalCfg) -> Self {
        Self { cfg, path: None }
    }
}

impl From<GlobalCfg> for FileCfg<GlobalCfg> {
    fn from(cfg: GlobalCfg) -> Self {
        Self { cfg, path: None }
    }
}

impl<C> Display for FileCfg<C>
where
    C: Serialize + DeserializeOwned,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(content) = serde_yaml::to_string(&self.cfg).map_err(|err| fmt::Error {}) {
            write!(f, "{}", content);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::cfg::file::FileCfg;
    use crate::cfg::{EnvPathsCfg, LocalCfg};
    use anyhow::{Context, Result};
    use predicates::prelude::Predicate;
    use predicates::str::contains;
    use short_utils::integration_test::environment::IntegrationTestEnvironment;
    use std::borrow::Borrow;

    fn init_env() -> IntegrationTestEnvironment {
        let mut e = IntegrationTestEnvironment::new("cmd_help");
        e.add_file("setup_1/template.yaml", "");
        e.add_file(
            "short.yml",
            r"#---
setups:
  - name: setup_1'
    public_env_directory: 'setup_1/'
    provider:
      name: cloudformation
      template: setup_1/template.yaml
#",
        );
        e.setup();
        e
    }

    #[test]
    fn file_cfg() {
        let e = init_env();
        let file_cfg: Result<FileCfg<LocalCfg>> = FileCfg::from_file(e.path().join("short.yml"));
        let file_cfg = file_cfg.unwrap();
        let path = file_cfg.path.unwrap().clone();
        assert!(contains("short.yml").eval(path.to_string_lossy().as_ref()));
    }

    #[test]
    fn local_file_cfg() {
        let e = init_env();
        let file_local_cfg =
            FileCfg::<LocalCfg>::from_local_file(e.path().join("setup_1"), "short.yml");
        let file_local_cfg = file_local_cfg.unwrap();
        let path = file_local_cfg.path.unwrap().clone();
        assert!(contains("short.yml").eval(path.to_string_lossy().as_ref()));
    }

    #[test]
    fn local_new_file() {
        let e = init_env();
    }
}
