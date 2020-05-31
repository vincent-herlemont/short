use crate::cli::terminal::emoji;
use anyhow::{Context, Result};
use clap::ArgMatches;
use log::*;

pub struct Settings {
    setup: Option<String>,
    env: Option<String>,
}

impl Settings {
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

pub fn get_settings(app: &ArgMatches) -> Settings {
    let mut settings = Settings {
        setup: None,
        env: None,
    };
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
