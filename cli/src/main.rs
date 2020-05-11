#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use std::env;

use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use terminal::emoji;

use crate::settings::Settings;

mod commands;
mod settings;
mod terminal;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const BIN_NAME: &'static str = "short";

fn main() -> Result<()> {
    env_logger::init();
    info!("BIN_NAME {}", BIN_NAME);
    info!("VERSION v{}", VERSION);
    Ok(run()?)
}

fn run() -> Result<()> {
    let app = App::new(format!("{} short", emoji::PARASOL))
        .version(VERSION)
        .author("Vincent Herlemont <vincentherl@leszeros.com>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("setup")
                .long("setup")
                .short("s")
                .takes_value(true)
                .global(true)
                .help("Set up name"),
        )
        .arg(
            Arg::with_name("environment")
                .long("env")
                .short("e")
                .takes_value(true)
                .global(true)
                .help("Environment name"),
        )
        .arg(
            Arg::with_name("dry-run")
                .long("dry-run")
                .global(true)
                .help("Disable all executions"),
        )
        .subcommand(
            SubCommand::with_name("init").about("Create an empty \"short.yml\" configuration file"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Add set up provider (cloudformation,...)")
                .subcommand(
                    SubCommand::with_name("cloudformation")
                        .about("Add cloudformation setup")
                        .arg(
                            Arg::with_name("template")
                                .help("The template cloudformation path")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("name")
                                .help("Name of your setup")
                                .long("name")
                                .short("n")
                                .takes_value(true),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("env")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .setting(AppSettings::DeriveDisplayOrder)
                .about("Manage environment")
                .subcommand(
                    SubCommand::with_name("new")
                        .about("Add new environment")
                        .arg(
                            Arg::with_name("name")
                                .help("name of your environment")
                                .index(1)
                                .required(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("dir")
                        .about("Change env directory")
                        .arg(
                            Arg::with_name("env_directory")
                                .help("Env directory path, must be inside of your project")
                                .index(1)
                                .required(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("pdir")
                        .about("Add or change private env directory")
                        .arg(
                            Arg::with_name("private_env_directory")
                                .help("Private env directory path, must be outside of your project")
                                .index(1)
                                .required(true),
                        ),
                ),
        )
        .subcommand(SubCommand::with_name("deploy").about("Deploy your set up"))
        .subcommand(SubCommand::with_name("show").about("Show your current set up"))
        .subcommand(
            SubCommand::with_name("use")
                .about("Switch of current setup or/and environment")
                .arg(
                    Arg::with_name("setup_name")
                        .help("The setup name to switch in")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("env_name")
                        .help("The environment name to switch in")
                        .index(2),
                ),
        )
        .subcommand(SubCommand::with_name("ls").about("List set up and environments"))
        .get_matches();

    let _settings = settings(&app);

    if let Some(_) = app.subcommand_matches("ls") {
        commands::ls()?;
    }

    Ok(())
}

fn settings(app: &ArgMatches) -> Settings {
    let mut settings = Settings::new();
    if let Some(setup) = app.value_of_lossy("setup") {
        settings.set_setup(setup.to_string());
    }
    info!("setup {:?}", settings.setup());
    if let Some(env) = app.value_of_lossy("env") {
        settings.set_env(env.to_string());
    }
    info!("env {:?}", settings.env());
    settings
}
