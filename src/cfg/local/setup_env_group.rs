use serde::de::{MapAccess, Visitor};
use serde::export::Formatter;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

type VarName = String;
type Pattern = String;

#[derive(Debug)]
pub struct EnvGroups(Rc<RefCell<Vec<EnvGroup>>>);

impl EnvGroups {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(vec![])))
    }

    pub fn add(&mut self, name: VarName, pattern: Pattern) {
        if self
            .0
            .borrow()
            .iter()
            .find(|env_group| env_group.0 == name)
            .is_none()
        {
            self.0.borrow_mut().append(&mut vec![(name, pattern)])
        }
    }

    pub fn inner(&self) -> Rc<RefCell<Vec<EnvGroup>>> {
        self.0.clone()
    }
}

impl Clone for EnvGroups {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl Serialize for EnvGroups {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let vec = self.0.borrow();
        let mut seq = serializer.serialize_map(Some(vec.len()))?;
        for e in vec.iter() {
            seq.serialize_entry(&e.0, &e.1)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for EnvGroups {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = EnvGroups;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("incorrect list of env_groups")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut env_groups = EnvGroups::new();
                while let Some((var_name, pattern)) = map.next_entry::<VarName, Pattern>()? {
                    env_groups.add(var_name, pattern);
                }
                Ok(env_groups)
            }
        }

        deserializer.deserialize_map(InnerVisitor)
    }
}

pub type EnvGroup = (VarName, Pattern);
