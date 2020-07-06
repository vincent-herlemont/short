use crate::cfg::global::setup::{GlobalProjectSetupCfg, SetupName};
use crate::cfg::SetupsCfg;
use anyhow::{Context, Result};
use serde::de::{MapAccess, Visitor};
use serde::export::Formatter;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::fmt;
use std::path::{Path, PathBuf};
use std::rc::Rc;

type EnvName = String;

#[derive(Debug, Serialize, Deserialize)]
struct CurrentSetup {
    #[serde(rename = "setup", skip_serializing_if = "Option::is_none")]
    pub setup_name: Option<String>,
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env_name: Option<EnvName>,
}

impl CurrentSetup {
    pub fn new() -> Self {
        Self {
            setup_name: None,
            env_name: None,
        }
    }
}

impl Default for CurrentSetup {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalProjectCfg {
    file: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    current: Option<CurrentSetup>,
    setups: GlobalProjectSetupsCfg,
}

#[derive(Debug)]
pub struct GlobalProjectSetupsCfg(Rc<RefCell<Vec<Rc<RefCell<GlobalProjectSetupCfg>>>>>);

impl GlobalProjectSetupsCfg {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(vec![])))
    }

    pub fn add(&mut self, global_setup_cfg: GlobalProjectSetupCfg) {
        let mut global_setups_cfg = self.0.borrow_mut();
        if global_setups_cfg
            .iter()
            .find(|lsc| {
                let lsc = lsc.borrow();
                lsc.name() == global_setup_cfg.name()
            })
            .is_none()
        {
            global_setups_cfg.push(Rc::new(RefCell::new(global_setup_cfg)))
        }
    }
}

impl Serialize for GlobalProjectSetupsCfg {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let vec = &self.0.borrow();
        let mut seq = serializer.serialize_map(Some(vec.len()))?;
        for global_setup_cfg in vec.iter() {
            let global_setup_cfg = global_setup_cfg.borrow();
            let name = global_setup_cfg.name();
            seq.serialize_entry(name, &*global_setup_cfg)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for GlobalProjectSetupsCfg {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = GlobalProjectSetupsCfg;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("incorrect list of global setup cfg")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut global_setups_cfg = GlobalProjectSetupsCfg::new();
                while let Some((setup_name, mut global_setup_cfg)) =
                    map.next_entry::<SetupName, GlobalProjectSetupCfg>()?
                {
                    global_setup_cfg.set_name(setup_name);
                    global_setups_cfg.add(global_setup_cfg);
                }
                Ok(global_setups_cfg)
            }
        }

        deserializer.deserialize_map(InnerVisitor)
    }
}

impl GlobalProjectCfg {
    pub fn new(file: &PathBuf) -> Result<Self> {
        let mut gp = GlobalProjectCfg {
            file: PathBuf::new(),
            current: None,
            setups: GlobalProjectSetupsCfg::new(),
        };
        gp.set_file(file)?;
        Ok(gp)
    }

    pub fn set_file(&mut self, file: &PathBuf) -> Result<()> {
        if !file.is_absolute() {
            return Err(anyhow!(format!(
                "project file path can not be relative {}",
                file.to_string_lossy()
            )));
        }
        if let None = file.file_name() {
            return Err(anyhow!(format!("project file has no name")));
        }
        self.file = file.clone();
        Ok(())
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn dir(&self) -> Result<&Path> {
        self.file.parent().context(format!(
            "fail to found parent directory of project `{}`",
            self.file.to_string_lossy()
        ))
    }

    pub fn set_current_setup_name(&mut self, setup_name: SetupName) {
        self.current.get_or_insert(CurrentSetup::new()).setup_name = Some(setup_name);
    }

    pub fn current_setup_name(&self) -> Option<&SetupName> {
        self.current
            .as_ref()
            .map_or(None, |current| current.setup_name.as_ref())
    }

    pub fn set_current_env_name(&mut self, env_name: EnvName) {
        self.current.get_or_insert(CurrentSetup::new()).env_name = Some(env_name);
    }

    pub fn current_env_name(&self) -> Option<&EnvName> {
        self.current
            .as_ref()
            .map_or(None, |current| current.env_name.as_ref())
    }

    pub fn unset_current_setup(&mut self) {
        self.current = None;
    }
}

impl SetupsCfg for GlobalProjectCfg {
    type Setup = GlobalProjectSetupCfg;

    fn get_setups(&self) -> Rc<RefCell<Vec<Rc<RefCell<Self::Setup>>>>> {
        Rc::clone(&self.setups.0)
    }
}

impl PartialEq<PathBuf> for GlobalProjectCfg {
    fn eq(&self, path_buf: &PathBuf) -> bool {
        self.file().eq(path_buf)
    }
}
impl PartialEq<GlobalProjectCfg> for PathBuf {
    fn eq(&self, path_buf: &GlobalProjectCfg) -> bool {
        self.eq(&path_buf.file)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::cfg::global::project::GlobalProjectCfg;
    use crate::cfg::global::setup::GlobalProjectSetupCfg;
    use crate::cfg::SetupsCfg;

    #[test]
    fn deserialization_serialization_cfg() {
        let content = r"---
file: path/to/file
current:
  setup: setup_1
setups:
  test_1: {}";
        let cfg = serde_yaml::from_str::<GlobalProjectCfg>(content).unwrap();
        let r = serde_yaml::to_string(&cfg).unwrap();
        assert_eq!(content, r);
    }

    #[test]
    fn global_update_private_env_dir() {
        let setup_cfg = GlobalProjectSetupCfg::new("setup".into());

        let mut project_cfg = GlobalProjectCfg::new(&"/project".into()).unwrap();
        project_cfg.add_setup(setup_cfg);

        assert!(project_cfg.get_setups().borrow().iter().count().eq(&1));

        {
            let setup_cfg = project_cfg.get_setup(&"setup".into()).unwrap();
            setup_cfg
                .borrow_mut()
                .set_private_env_dir("/private_env".into())
                .unwrap();
        }

        let global_project_setup_cfg_1 = project_cfg.get_setup(&"setup".into()).unwrap();
        assert_eq!(
            global_project_setup_cfg_1
                .borrow()
                .private_env_dir()
                .unwrap(),
            &PathBuf::from("/private_env")
        );

        project_cfg.remove_by_name_setup(&"setup".into());
        assert!(project_cfg.get_setup(&"setup".into()).is_none());
    }
}
