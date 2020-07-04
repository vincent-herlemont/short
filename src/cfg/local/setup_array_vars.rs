use std::fmt;

use anyhow::Result;

use serde::de;
use serde::de::{MapAccess, Unexpected, Visitor};
use serde::export::Formatter;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use strum;
use strum::EnumProperty;
use strum_macros::EnumProperty;
use strum_macros::EnumString;

use crate::cfg::local::VarName;

type VarPattern = String;

#[derive(EnumString, EnumProperty, Debug, Clone, Eq, PartialEq)]
pub enum VarCase {
    #[strum(
        serialize = "None",
        serialize = "none",
        serialize = "false",
        serialize = "",
        props(deserialize = "")
    )]
    None,
    #[strum(
        serialize = "camelcase",
        serialize = "CamelCase",
        props(deserialize = "CamelCase")
    )]
    CamelCase,
    #[strum(
        serialize = "snakecase",
        serialize = "snake_case",
        props(deserialize = "snake_case")
    )]
    SnakeCase,
    #[strum(
        serialize = "kebabcase",
        serialize = "kebab-case",
        props(deserialize = "kebab-case")
    )]
    KebabCase,
    #[strum(
        serialize = "shoutyshnakecase",
        serialize = "SHOUTY_SNAKE_CASE",
        props(deserialize = "SHOUTY_SNAKE_CASE")
    )]
    ShoutySnakeCase,
    #[strum(
        serialize = "mixedcase",
        serialize = "mixedCase",
        props(deserialize = "mixedCase")
    )]
    MixedCase,
    #[strum(
        serialize = "titlecase",
        serialize = "Title Case",
        props(deserialize = "Title Case")
    )]
    TitleCase,
}

impl Serialize for VarCase {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let str = self.get_str("deserialize").unwrap_or("");
        serializer.serialize_str(&str)
    }
}

impl<'de> Deserialize<'de> for VarCase {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = VarCase;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("incorrect list of var_format")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Ok(var_format) = VarCase::from_str(v) {
                    Ok(var_format)
                } else {
                    Err(de::Error::invalid_type(Unexpected::Str(v), &self))
                }
            }
        }

        deserializer.deserialize_string(InnerVisitor)
    }
}

impl Default for VarCase {
    fn default() -> Self {
        VarCase::None
    }
}

impl VarCase {
    pub fn is_none(&self) -> bool {
        matches!(*self, VarCase::None)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArrayVars(Vec<ArrayVar>);

impl ArrayVars {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, array_var: ArrayVar) {
        if self.0.iter().find(|av| av.name == array_var.name).is_none() {
            // self.0.append(&mut vec![array_var])
            self.0.push(array_var)
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
        for array_var in vec.iter() {
            seq.serialize_entry(&array_var.name, &array_var)?;
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
                while let Some((var_name, mut array_var)) = map.next_entry::<VarName, ArrayVar>()? {
                    array_var.name = var_name;
                    array_vars.add(array_var);
                }
                Ok(array_vars)
            }
        }

        deserializer.deserialize_map(InnerVisitor)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArrayVar {
    name: VarName,
    pattern: VarPattern,
    case: VarCase,
}

impl From<DeserializeArrayVarTruncate> for ArrayVar {
    fn from(davt: DeserializeArrayVarTruncate) -> Self {
        ArrayVar::new("".into(), davt.pattern, davt.case)
    }
}

#[derive(Serialize, Deserialize)]
struct DeserializeArrayVarTruncate {
    pattern: VarPattern,
    #[serde(skip_serializing_if = "VarCase::is_none", default = "VarCase::default")]
    case: VarCase,
}

impl From<ArrayVar> for DeserializeArrayVarTruncate {
    fn from(av: ArrayVar) -> Self {
        Self {
            pattern: av.pattern,
            case: av.case,
        }
    }
}

impl Serialize for ArrayVar {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match &self.case {
            VarCase::None => serializer.serialize_str(&self.pattern),
            _ => {
                let davt = DeserializeArrayVarTruncate::from(self.clone());
                serializer.serialize_newtype_struct("", &davt)
            }
        }
    }
}

impl<'de> Deserialize<'de> for ArrayVar {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = ArrayVar;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("incorrect list of array_var")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ArrayVar::new("".into(), v.into(), VarCase::None))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mvd = de::value::MapAccessDeserializer::new(map);
                DeserializeArrayVarTruncate::deserialize(mvd).map(|davt| davt.into())
            }
        }

        deserializer.deserialize_any(InnerVisitor)
    }
}

impl ArrayVar {
    pub fn new(name: VarName, pattern: VarPattern, format: VarCase) -> Self {
        Self {
            name,
            pattern,
            case: format,
        }
    }

    pub fn var(&self) -> &VarName {
        &self.name
    }

    pub fn pattern(&self) -> &VarPattern {
        &self.pattern
    }
    pub fn case(&self) -> &VarCase {
        &self.case
    }
}

#[cfg(test)]
mod tests {

    use crate::cfg::local::setup_array_vars::VarCase;
    use crate::cfg::{ArrayVar, ArrayVars};
    use serde_yaml;

    #[test]
    fn deserialize_array_vars() {
        let content = r#"
test_1: value_1
test_2: value_2
test_3:
    pattern: value_3
test_4:
    pattern: value_4
    case: kebab-case
        "#;

        let array_vars = serde_yaml::from_str::<ArrayVars>(content).unwrap();

        let mut expected_array_vars = ArrayVars::new();
        expected_array_vars.add(ArrayVar::new(
            "test_1".into(),
            "value_1".into(),
            VarCase::None,
        ));
        expected_array_vars.add(ArrayVar::new(
            "test_2".into(),
            "value_2".into(),
            VarCase::None,
        ));
        expected_array_vars.add(ArrayVar::new(
            "test_3".into(),
            "value_3".into(),
            VarCase::None,
        ));
        expected_array_vars.add(ArrayVar::new(
            "test_4".into(),
            "value_4".into(),
            VarCase::KebabCase,
        ));
        assert_eq!(array_vars, expected_array_vars);

        let output_content = serde_yaml::to_string(&array_vars).unwrap();
        assert_eq!(
            output_content,
            r"---
test_1: value_1
test_2: value_2
test_3: value_3
test_4:
  pattern: value_4
  case: kebab-case"
        );
    }
}
