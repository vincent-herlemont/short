use crate::cfg::local::Var;
use serde::de::{MapAccess, Visitor};
use serde::export::Formatter;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

type Pattern = String;

#[derive(Debug, Clone)]
pub struct ArrayVars(Vec<ArrayVar>);

impl ArrayVars {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, name: Var, pattern: Pattern) {
        if self
            .0
            .iter()
            .find(|array_var| array_var.0 == name)
            .is_none()
        {
            self.0.append(&mut vec![(name, pattern).into()])
        }
    }
}

impl AsRef<Vec<ArrayVar>> for ArrayVars {
    fn as_ref(&self) -> &Vec<ArrayVar> {
        &self.0
    }
}

impl Default for ArrayVars {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for ArrayVars {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let vec = &self.0;
        let mut seq = serializer.serialize_map(Some(vec.len()))?;
        for e in vec.iter() {
            seq.serialize_entry(&e.0, &e.1)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for ArrayVars {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = ArrayVars;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("incorrect list of array_vars")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut array_vars = ArrayVars::new();
                while let Some((var, pattern)) = map.next_entry::<Var, Pattern>()? {
                    array_vars.add(var, pattern);
                }
                Ok(array_vars)
            }
        }

        deserializer.deserialize_map(InnerVisitor)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ArrayVar(Var, Pattern);

impl ArrayVar {
    pub fn var(&self) -> &Var {
        &self.0
    }

    pub fn pattern(&self) -> &Pattern {
        &self.1
    }
}

impl From<(Var, Pattern)> for ArrayVar {
    fn from(t: (Var, Pattern)) -> Self {
        Self(t.0, t.1)
    }
}
