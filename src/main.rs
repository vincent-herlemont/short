#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use anyhow::Result;
use clap::{App, AppSettings, Arg, SubCommand};
use cli::commands;
use cli::terminal::emoji;
use short::*;
use std::env;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const BIN_NAME: &'static str = "short";

fn main() -> Result<()> {
    env_logger::init();
    info!("BIN_NAME {}", BIN_NAME);
    info!("VERSION v{}", VERSION);
    Ok(run()?)
}

fn run() -> Result<()> {
    let _setup_arg = Arg::with_name("setup")
        .long("setup")
        .short("s")
        .takes_value(true)
        .help("Set up name");
    let _environment_arg = Arg::with_name("environment")
        .long("env")
        .short("e")
        .takes_value(true)
        .help("Environment name");
    let _dryrun_arg = Arg::with_name("dry-run")
        .long("dry-run")
        .help("Disable all executions");

    let app = App::new(format!("{} short", emoji::PARASOL))
        .version(VERSION)
        .author("Vincent Herlemont <vincentherl@leszeros.com>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("init")
                .about("Init project, create an empty \"short.yml\" configuration file"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Create new setup")
                .arg(
                    Arg::with_name("setup_name")
                        .index(1)
                        .required(true)
                        .help("Setup name"),
                )
                .arg(
                    Arg::with_name("file")
                        .long("file")
                        .short("f")
                        .default_value("run.sh")
                        .help("Path script"),
                )
                .arg(
                    Arg::with_name("shebang")
                        .long("shebang")
                        .short("s")
                        .default_value("#!/bin/bash")
                        .help("Interpreter program"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rename")
                .about("Rename setup")
                .arg(
                    Arg::with_name("last_setup_name")
                        .index(1)
                        .required(true)
                        .help("Last setup name"),
                )
                .arg(
                    Arg::with_name("new_setup_name")
                        .index(2)
                        .required(true)
                        .help("New setup name"),
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

    if let Some(_) = app.subcommand_matches("init") {
        commands::init(&app)?;
    } else if let Some(args) = app.subcommand_matches("new") {
        commands::new(&args)?;
    } else if let Some(_) = app.subcommand_matches("ls") {
        commands::ls(&app)?;
    } else if let Some(args) = app.subcommand_matches("rename") {
        commands::rename(&args)?;
    }

    Ok(())
}
