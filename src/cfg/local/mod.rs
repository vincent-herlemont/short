use std::cell::RefCell;
use std::rc::Rc;

use serde::de::{MapAccess, Visitor};
use serde::export::Formatter;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub use setup::LocalSetupCfg;
pub use setup_array_vars::{ArrayVar, ArrayVars, VarCase};
pub use setup_vars::{VarName, Vars};

use crate::cfg::local::setup::SetupName;
use crate::cfg::setup::SetupsCfg;
use crate::cfg::SetupCfg;

mod setup;
mod setup_array_vars;
mod setup_vars;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalCfg {
    setups: LocalSetupsCfg,
}

impl LocalCfg {
    pub fn new() -> Self {
        Self {
            setups: LocalSetupsCfg::new(),
        }
    }
}

impl SetupsCfg for LocalCfg {
    type Setup = LocalSetupCfg;

    fn get_setups(&self) -> Rc<RefCell<Vec<Rc<RefCell<LocalSetupCfg>>>>> {
        Rc::clone(&self.setups.0)
    }
}

#[derive(Debug)]
pub struct LocalSetupsCfg(Rc<RefCell<Vec<Rc<RefCell<LocalSetupCfg>>>>>);

impl LocalSetupsCfg {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(vec![])))
    }
    pub fn add(&mut self, local_setup_cfg: LocalSetupCfg) {
        let mut local_setups_cfg = self.0.borrow_mut();
        if local_setups_cfg
            .iter()
            .find(|lsc| {
                let lsc = lsc.borrow();
                lsc.name() == local_setup_cfg.name()
            })
            .is_none()
        {
            local_setups_cfg.push(Rc::new(RefCell::new(local_setup_cfg)))
        }
    }
}

impl Serialize for LocalSetupsCfg {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let vec = &self.0.borrow();
        let mut seq = serializer.serialize_map(Some(vec.len()))?;
        for local_setup_cfg in vec.iter() {
            let local_setup_cfg = local_setup_cfg.borrow();
            let name = local_setup_cfg.name();
            seq.serialize_entry(name, &*local_setup_cfg)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for LocalSetupsCfg {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = LocalSetupsCfg;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("incorrect list of local setup cfg")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut local_setups_cfg = LocalSetupsCfg::new();
                while let Some((setup_name, mut local_setup_cfg)) =
                    map.next_entry::<SetupName, LocalSetupCfg>()?
                {
                    local_setup_cfg.set_name(setup_name.to_owned());
                    local_setups_cfg.add(local_setup_cfg);
                }
                Ok(local_setups_cfg)
            }
        }

        deserializer.deserialize_map(InnerVisitor)
    }
}

#[cfg(test)]
mod tests {

    use crate::cfg::setup::SetupsCfg;
    use crate::cfg::{LocalCfg, LocalSetupCfg};

    #[test]
    fn deserialization_serialization_local_cfg() {
        let content = r#"---
setups:
  test1:
    file: run.sh"#;

        let cfg = serde_yaml::from_str::<LocalCfg>(content).unwrap();
        let r = serde_yaml::to_string(&cfg).unwrap();
        assert_eq!(r, content);
    }

    #[test]
    fn local_update_public_env_dir() {
        let setup_cfg = LocalSetupCfg::new("setup".into(), "run.sh".into());

        let mut local_cfg = LocalCfg::new();
        local_cfg.add_setup(setup_cfg);

        {
            let setup_cfg_1 = local_cfg.get_setup(&"setup".into()).unwrap();
            let mut setup_cfg_1 = setup_cfg_1.borrow_mut();
            setup_cfg_1.set_public_env_dir("./env_dir/".into());
        }

        local_cfg.remove_by_name_setup(&"setup".into());
        assert!(local_cfg.get_setup(&"setup".into()).is_none());
    }
}
