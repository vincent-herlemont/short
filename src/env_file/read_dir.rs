use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::env_file::Env;

pub fn read_dir(dir: &PathBuf) -> Vec<Result<Env>> {
    let mut envs = vec![];
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if !entry.path().is_file() {
                    continue;
                }

                // Ignore files that not start by "."
                if let Ok(file_name) = entry.file_name().into_string() {
                    if !file_name.starts_with(".") {
                        continue;
                    }
                } else {
                    continue;
                }

                let path = entry.path();
                let env = Env::from_file_reader(&path)
                    .context(format!("fail to read read `{:?}` ", &path));
                envs.append(&mut vec![env]);
            }
        }
    }
    envs
}
