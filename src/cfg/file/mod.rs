use crate::cfg::global::GLOBAL_FILE_NAME;
use crate::cfg::{GlobalCfg, LocalCfg};
use crate::utils::find::find_in_parents;
use crate::utils::write_all::write_all_dir;
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env::var;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileCfg<C>
where
    C: Serialize + DeserializeOwned,
{
    file: Option<PathBuf>,
    cfg: C,
}

impl<C> FileCfg<C>
where
    C: Serialize + DeserializeOwned,
{
    pub fn load(file: &PathBuf) -> Result<FileCfg<C>> {
        let str = read_to_string(&file)?;
        let cfg = serde_yaml::from_str(str.as_str())
            .context(format!("fail to parse {}", file.to_string_lossy()))?;
        Ok(Self {
            cfg,
            file: Some(file.to_path_buf()),
        })
    }

    pub fn new(file: &PathBuf, cfg: C) -> Result<FileCfg<C>> {
        if !file.is_absolute() {
            return Err(anyhow!("cfg file path must be an abosulte path"));
        }
        Ok(Self {
            file: Some(file.to_path_buf()),
            cfg,
        })
    }

    pub fn save(&self) -> Result<()> {
        let path = self.file.as_ref().context("cfg file has not path")?;
        let str = serde_yaml::to_string(&self.cfg).context("fail to parse cfg")?;
        write_all_dir(path, &str).context("fail to write cfg file")?;
        Ok(())
    }

    pub fn file(&self) -> Result<&PathBuf> {
        self.file.as_ref().context("local cfg has no path")
    }

    pub fn borrow(&self) -> &C {
        &self.cfg
    }

    pub fn borrow_mut(&mut self) -> &mut C {
        &mut self.cfg
    }
}

fn local_cfg_file(dir: &PathBuf) -> PathBuf {
    let local_cfg_file = var("SHORT_LOCAL_CFG_FILE").map_or("short.yaml".to_string(), |v| v);
    dir.join(local_cfg_file)
}

pub fn load_local_cfg(dir: &PathBuf) -> Result<FileCfg<LocalCfg>> {
    let local_cfg_file = local_cfg_file(dir);

    let local_cfg = get_local_cfg(&local_cfg_file).context(format!(
        "cfg file not found {}",
        local_cfg_file.to_string_lossy()
    ))?;

    Ok(local_cfg)
}

pub fn new_local_cfg(dir: &PathBuf) -> Result<FileCfg<LocalCfg>> {
    let local_cfg_file = local_cfg_file(dir);

    if let Ok(_) = get_local_cfg(&local_cfg_file) {
        return Err(anyhow!("local cfg {:?} already exist", local_cfg_file));
    }

    FileCfg::new(&local_cfg_file, LocalCfg::new())
}

pub fn load_or_new_global_cfg(dir: &PathBuf) -> Result<FileCfg<GlobalCfg>> {
    let global_cfg_dir = var("SHORT_GLOBAL_CFG_DIR").map_or(".short/".to_string(), |v| v);
    let global_dir = dir.join(global_cfg_dir);
    let global_cfg_file = global_dir.join(GLOBAL_FILE_NAME.to_string());

    let global = get_global_cfg(&global_cfg_file).map_or(
        FileCfg::new(&global_cfg_file, GlobalCfg::new()).context(format!(
            "fail to create new global cfg file {:?}",
            global_cfg_file
        ))?,
        |v| v,
    );

    Ok(global)
}

pub fn get_local_cfg(file: &PathBuf) -> Result<FileCfg<LocalCfg>> {
    let dir = file.parent().context(format!(
        "fail to reach directory of local cfg file {}",
        file.to_string_lossy()
    ))?;
    let file_name = file
        .file_name()
        .context(format!(
            "fail te get file name of local cfg file {}",
            file.to_string_lossy()
        ))?
        .to_str()
        .context(format!(
            "cfg file name mut be contain only utf-8 char : {}",
            file.to_string_lossy()
        ))?
        .to_string();
    let path = find_in_parents(dir.to_path_buf(), file_name).context("fail to found local cfg")?;
    FileCfg::load(&path)
}

pub fn get_global_cfg(file: &PathBuf) -> Result<FileCfg<GlobalCfg>> {
    FileCfg::load(file)
}

impl From<LocalCfg> for FileCfg<LocalCfg> {
    fn from(cfg: LocalCfg) -> Self {
        Self { cfg, file: None }
    }
}

impl From<GlobalCfg> for FileCfg<GlobalCfg> {
    fn from(cfg: GlobalCfg) -> Self {
        Self { cfg, file: None }
    }
}

impl<C> Display for FileCfg<C>
where
    C: Serialize + DeserializeOwned,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(content) = serde_yaml::to_string(&self.cfg).map_err(|_err| fmt::Error {}) {
            write!(f, "{}", content);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::cfg::file::{load_local_cfg, FileCfg};
    use crate::cfg::LocalCfg;
    use anyhow::Result;
    use cli_integration_test::IntegrationTestEnvironment;
    use predicates::prelude::Predicate;
    use predicates::str::contains;
    use std::path::PathBuf;

    fn init_env() -> IntegrationTestEnvironment {
        let mut e = IntegrationTestEnvironment::new("cmd_help");
        e.add_file("setup_1/template.yaml", "");
        e.add_file(
            "short.yaml",
            r"#---
setups:
  - name: setup_1'
    public_env_dir: 'setup_1/'
    file: ./run.sh
#",
        );
        e.setup();
        e
    }

    #[test]
    fn load() {
        let e = init_env();
        let file_cfg: Result<FileCfg<LocalCfg>> = FileCfg::load(&e.path().join("short.yaml"));
        let file_cfg = file_cfg.unwrap();
        let path = file_cfg.file.unwrap().clone();
        assert!(contains("short.yaml").eval(path.to_string_lossy().as_ref()));
    }

    #[test]
    fn load_local() {
        let e = init_env();
        let file_local_cfg = load_local_cfg(&e.path().join("setup_1/short.yaml"));
        let file_local_cfg = file_local_cfg.unwrap();
        let path = file_local_cfg.file.unwrap().clone();
        assert!(contains("short.yaml").eval(path.to_string_lossy().as_ref()));
    }

    #[test]
    fn local_new_file() {
        let e = init_env();
        let local_cfg = LocalCfg::new();

        FileCfg::new(&PathBuf::from("example"), local_cfg).unwrap_err();

        let local_cfg = LocalCfg::new();
        let _file_cfg_local =
            FileCfg::new(&e.path().join(PathBuf::from("example")), local_cfg).unwrap();
    }
}
