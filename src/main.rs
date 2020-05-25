#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use cli::commands;
use cli::settings::Settings;
use cli::terminal::emoji;
use dirs::home_dir;
use short::cfg::Cfg;
use short::cli::settings::get_settings;
use short::*;
use std::env;
use std::env::current_dir;
use std::path::PathBuf;

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

    let _settings = get_settings(&app);

    if let Some(_) = app.subcommand_matches("ls") {
        commands::ls()?;
    }

    Ok(())
}
