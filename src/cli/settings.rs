use std::fmt::Display;

use anyhow::{Context, Result};
use clap::ArgMatches;
use log::*;
use serde::export::Formatter;

use crate::cfg::Cfg;
use crate::cli::terminal::emoji;

#[derive(Debug)]
pub struct Settings {
    setup: Option<String>,
    env: Option<String>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            setup: None,
            env: None,
        }
    }

    pub fn set_setup(&mut self, setup: String) {
        self.setup = Some(setup);
    }

    pub fn set_env(&mut self, env: String) {
        self.env = Some(env)
    }

    pub fn setup(&self) -> Result<&String> {
        self.setup.as_ref().context(format!(
            r#"setup not specified: 
{0} you can set a current setup with the command \"short use <setup> <environment>\".
{0} you can use the \"-s <setup>\" argument."#,
            emoji::RIGHT_POINTER
        ))
    }

    pub fn env(&self) -> Result<&String> {
        self.env.as_ref().context(format!(
            r#"env not specified: 
{0} you can set a current env with the command \"short use <setup> <environment>\".
{0} you can use the \"-e <env>\" argument."#,
            emoji::RIGHT_POINTER
        ))
    }
}

impl Display for Settings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(setup) = &self.setup {
            write!(f, "{}", setup)?;
            if let Some(env) = &self.env {
                write!(f, ":{}", env)?;
            }
        } else {
            write!(f, "<unknown_setup>")?;
        }
        Ok(())
    }
}

pub fn get_settings(app: &ArgMatches, cfg: &Cfg) -> Settings {
    let mut settings: Settings = cfg.into();

    if let Some(setup) = app.value_of_lossy("setup") {
        settings.set_setup(setup.to_string());
        info!("setup {:?}", setup);
    }
    if let Some(env) = app.value_of_lossy("environment") {
        settings.set_env(env.to_string());
        info!("env {:?}", env);
    }
    settings
}

impl From<&Cfg> for Settings {
    fn from(cfg: &Cfg) -> Self {
        if let Ok(current_project) = cfg.current_project() {
            let mut settings = Settings::new();
            let current_project = current_project.borrow();

            if let Some(setup_name) = current_project.current_setup_name() {
                settings.set_setup(setup_name.clone())
            }

            if let Some(env_name) = current_project.current_env_name() {
                settings.set_env(env_name.clone());
            }

            settings
        } else {
            Self::new()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cli::settings::Settings;

    #[test]
    fn settings() {
        let mut s = Settings {
            setup: None,
            env: None,
        };
        s.set_setup("setup".to_string());
        s.set_env("env".to_string());
        assert_eq!(&"setup".to_string(), s.setup().unwrap());
        assert_eq!(&"env".to_string(), s.env().unwrap());
    }
}
