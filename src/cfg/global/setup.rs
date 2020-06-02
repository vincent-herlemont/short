use crate::cfg::{CfgError, LocalSetupCfg, SetupCfg};
use anyhow::Result;
use serde::de::{Unexpected, Visitor};
use serde::export::Formatter;
use serde::{Deserialize, Deserializer, Serialize};
use std::path::{PathBuf};
use std::result::Result as stdResult;

#[derive(Debug, Serialize, Deserialize)]
struct PrivateEnvDir(#[serde(deserialize_with = "deserialize_private_env_dir")] PathBuf);

impl AsRef<PathBuf> for PrivateEnvDir {
    fn as_ref(&self) -> &PathBuf {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalProjectSetupCfg {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    private_env_dir: Option<PrivateEnvDir>,
}

impl GlobalProjectSetupCfg {
    pub fn new(name: String) -> Self {
        Self {
            name,
            private_env_dir: None,
        }
    }

    pub fn private_env_dir(&self) -> Result<&PathBuf> {
        self.private_env_dir
            .as_ref()
            .map(|private_env_dir| private_env_dir.as_ref())
            .ok_or(CfgError::PrivateEnvDirNotFound(self.name.clone()).into())
    }

    pub fn set_private_env_dir(&mut self, dir: PathBuf) -> Result<()> {
        if dir.is_relative() {
            bail!(CfgError::PrivateEnvDirRelativePath(dir, self.name.clone()))
        }
        self.private_env_dir = Some(PrivateEnvDir(dir));
        Ok(())
    }

    pub fn unset_private_env_dir(&mut self) -> Result<()> {
        if let None = self.private_env_dir {
            bail!(CfgError::PrivateEnvAlreadyUnset(self.name.clone()))
        } else {
            self.private_env_dir = None;
            Ok(())
        }
    }
}

impl From<&LocalSetupCfg> for GlobalProjectSetupCfg {
    fn from(local_setup: &LocalSetupCfg) -> Self {
        Self {
            name: local_setup.name().clone(),
            private_env_dir: None,
        }
    }
}

impl SetupCfg for GlobalProjectSetupCfg {
    fn name(&self) -> &String {
        &self.name
    }

    fn rename(&mut self, name: &String) {
        self.name = name.clone();
    }
}

fn deserialize_private_env_dir<'de, D>(deserializer: D) -> stdResult<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    struct InnerVisitor;

    impl<'de> Visitor<'de> for InnerVisitor {
        type Value = PathBuf;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("incorrect private_env_dir")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let dir: PathBuf = v.into();
            if dir.is_relative() {
                Err(E::invalid_value(
                    Unexpected::Str(v),
                    &"private_env_dir must be an absolute path",
                ))
            } else {
                Ok(dir)
            }
        }
    }

    deserializer.deserialize_str(InnerVisitor)
}

#[cfg(test)]
mod tests {
    use crate::cfg::global::GlobalProjectSetupCfg;
    use predicates::prelude::Predicate;
    use predicates::str::contains;
    use serde_yaml;
    use std::path::PathBuf;

    #[test]
    fn deserialize_private_env_dir() {
        let content = r#"
name: setup_1
        "#;
        let setup_cfg = serde_yaml::from_str::<GlobalProjectSetupCfg>(content).unwrap();
        assert!(setup_cfg.private_env_dir().is_err());

        let content = r#"
name: setup_1
private_env_dir: ./rel_path
        "#;
        let error = serde_yaml::from_str::<GlobalProjectSetupCfg>(content).unwrap_err();
        assert!(contains("private_env_dir must be an absolute path").eval(&error.to_string()));

        let content = r#"
name: setup_1
private_env_dir: /rel_path
        "#;
        let setup_cfg = serde_yaml::from_str::<GlobalProjectSetupCfg>(content).unwrap();
        assert_eq!(
            &PathBuf::from("/rel_path"),
            setup_cfg.private_env_dir().unwrap()
        );
    }
}
