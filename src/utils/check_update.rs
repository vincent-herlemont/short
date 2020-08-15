#![cfg(feature = "reqwest")]
use crate::cli::terminal::emoji;
use crate::utils::file_time::create_time;
use anyhow::{Context, Result};
use colored::*;
use filetime;

use serde::de::{Unexpected, Visitor};
use serde::export::Formatter;
use serde::{de, Serialize, Serializer};
use serde::{Deserialize, Deserializer};
use serde_json::{from_str, to_string};
use std::cmp::Ordering;
use std::fmt;
use std::fs::remove_file;
use std::path::{Path, PathBuf};
use std::time::Duration;
use versions;

const GITHUB_CRATES_INDEX_FILE_URL: &'static str =
    "https://raw.githubusercontent.com/rust-lang/crates.io-index/master/sh/or/short";
const LAST_CRATE_INFO_FILE: &'static str = ".last_update.json";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CrateInfo {
    name: String,
    vers: Versioning,
}

impl CrateInfo {
    pub fn new(version: &str) -> Result<Self> {
        let name = env!("CARGO_PKG_NAME");
        let version = versions::Versioning::new(version).context("fail to parse version")?;
        Ok(Self {
            name: name.into(),
            vers: Versioning(version),
        })
    }

    pub fn current() -> CrateInfo {
        let version = env!("CARGO_PKG_VERSION");
        CrateInfo::new(version).unwrap()
    }
}

impl PartialEq<CrateInfo> for CrateInfo {
    fn eq(&self, other: &CrateInfo) -> bool {
        self.vers.0 == other.vers.0
    }
}

impl PartialOrd for CrateInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.vers.0.partial_cmp(&other.vers.0)
    }
}

#[derive(Debug, Clone)]
struct Versioning(versions::Versioning);

impl<'de> Deserialize<'de> for Versioning {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = Versioning;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("incorrect list of global setup cfg")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                versions::Versioning::new(v)
                    .map(|v| Versioning(v))
                    .ok_or(de::Error::invalid_value(Unexpected::Str(v), &self))
            }
        }

        deserializer.deserialize_str(InnerVisitor)
    }
}

impl Serialize for Versioning {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let str = self.0.to_string();
        serializer.serialize_str(&str)
    }
}

use crate::utils::write_all::write_all_dir;
use reqwest::blocking::Client;

fn latest() -> Result<CrateInfo> {
    let client = Client::builder().timeout(Duration::from_secs(2)).build()?;
    let body = client.get(GITHUB_CRATES_INDEX_FILE_URL).send()?.text()?;
    let mut output: Option<CrateInfo> = None;
    for row in body.lines() {
        if let Ok(tmp_crate_info) = from_str::<CrateInfo>(row) {
            if let Some(crate_info) = output.as_ref() {
                if *crate_info < tmp_crate_info {
                    output = Some(tmp_crate_info.clone());
                }
            } else {
                output = Some(tmp_crate_info.clone());
            }
        }
    }
    output.context("no crate info")
}

fn file_name(dir: &Path) -> PathBuf {
    dir.join(LAST_CRATE_INFO_FILE)
}

fn is_checked(file: &Path, ttl: &i64) -> bool {
    if file.exists() {
        let current_time = filetime::FileTime::now();
        let create_time = create_time(&file);
        if create_time.seconds() + ttl > current_time.seconds() {
            return true;
        }
    }
    false
}

fn save(file: &Path, crate_info: &CrateInfo) -> Result<()> {
    let content = to_string(crate_info)?;
    write_all_dir(&file, &content)?;
    Ok(())
}

pub type UpdateMessage = String;

pub fn check_update(dir: &Path, current_version: &CrateInfo, ttl: &i64) -> Option<UpdateMessage> {
    let filename = file_name(dir);

    if !filename.exists() {
        if let Ok(_) = save(&filename, &current_version) {
            return None;
        }
    }

    if !is_checked(&filename, ttl) {
        if let Ok(latest_version) = latest() {
            if latest_version > *current_version {
                if let Ok(_) = remove_file(&filename) {
                    save(&filename, &latest_version).unwrap();
                }
                return Some(format!(
                    r#"
 {love} {t1} {version} {t2}
    For update you can use the this command :
 
 {hand} {command}
 
"#,
                    love = emoji::LOVE,
                    t1 = "New short version".green(),
                    version = latest_version.vers.0.to_string(),
                    t2 = "available !!".green(),
                    hand = emoji::RIGHT_POINTER,
                    command = "cargo install short -f".bold().purple()
                ));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::utils::check_update::{latest, CrateInfo};

    #[test]
    fn test_current_version() {
        CrateInfo::current();
    }

    #[test]
    fn test_last_version() {
        latest().unwrap();
    }

    #[test]
    fn test_compare_version() {
        let crate_info1 = CrateInfo::new("0.0.1").unwrap();
        let crate_info2 = CrateInfo::new("0.0.2").unwrap();
        assert!(crate_info1 < crate_info2);
        assert!(!(crate_info1 > crate_info2));
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::utils::check_update::{check_update, file_name, is_checked, save, CrateInfo};
    use cli_integration_test::IntegrationTestEnvironment;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_is_checked() {
        let e = IntegrationTestEnvironment::new("test_save_check");
        e.setup();

        let crate_info = CrateInfo::new("0.0.1").unwrap();
        let file_name = file_name(&e.path().unwrap());
        let ttl = &1;

        save(&file_name, &crate_info).unwrap();
        assert!(is_checked(&file_name, ttl));
        thread::sleep(Duration::from_secs(1));
        assert!(!is_checked(&file_name, ttl));
    }

    #[test]
    fn test_check_workflow() {
        let e = IntegrationTestEnvironment::new("test_save_check");
        e.setup();

        let crate_info = CrateInfo::new("0.0.1").unwrap();
        let file_name = file_name(&e.path().unwrap());
        let ttl = &1;
        let r = check_update(&e.path().unwrap(), &crate_info, ttl);
        assert!(r.is_none());
        let r = e.read_file(file_name);
        assert_eq!(r#"{"name":"short","vers":"0.0.1"}"#, r);
        let r = check_update(&e.path().unwrap(), &crate_info, ttl);
        assert!(r.is_none());
        thread::sleep(Duration::from_secs(1));
        let r = check_update(&e.path().unwrap(), &crate_info, ttl);
        assert!(r.is_some());
        let r = check_update(&e.path().unwrap(), &crate_info, ttl);
        assert!(r.is_none());
    }
}
