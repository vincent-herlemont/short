#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};
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
    let setup_arg = Arg::with_name("setup")
        .long("setup")
        .short("s")
        .takes_value(true)
        .help("Set up name");
    let environment_arg = Arg::with_name("environment")
        .long("env")
        .short("e")
        .takes_value(true)
        .help("Environment name");
    let dryrun_arg = Arg::with_name("dry-run")
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
                    Arg::with_name("env_name")
                        .index(2)
                        .required(true)
                        .help("Env Name"),
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
                )
                .arg(Arg::with_name("private").long("private").short("p").help("Save to private directory")),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Run setup")
                .arg(setup_arg.clone())
                .arg(environment_arg.clone())
                .arg(dryrun_arg.clone()),
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
                .about("Manage environment files")
                .subcommand(
                    SubCommand::with_name("new")
                        .about("Add new environment, create env file \".<environment>\", in public directory default.")
                        .arg(Arg::with_name("name")
                                .help("Environment name")
                                .index(1)
                                .required(true),
                        )
                        .arg(setup_arg.clone())
                        .arg(Arg::with_name("private").long("private").short("p").help("Save to private directory")),
                )
                .subcommand(SubCommand::with_name("edit")
                    .about("Edit env file")
                    .arg(Arg::with_name("environment")
                        .help("Environment name")
                        .index(1)
                    )
                    .arg(setup_arg.clone())
                    .arg(Arg::with_name("editor")
                        .long("editor")
                        .takes_value(true)
                        .help("Editor"))
                )
                .subcommand(
                    SubCommand::with_name("dir")
                        .about("Change env directory, [.] by default.")
                        .arg(
                            Arg::with_name("env_dir")
                                .help("Env directory path, must be directory child of your project")
                                .index(1)
                        )
                        .arg(Arg::with_name("unset").long("unset").help("Unset directory path"))
                        .group(ArgGroup::with_name("action").args(&["env_dir","unset"]).required(true))
                        .arg(setup_arg.clone()),
                )
                .subcommand(
                    SubCommand::with_name("pdir")
                        .about("Add or change private env directory")
                        .arg(
                            Arg::with_name("env_dir")
                                .help("Private env directory path, must be outside of your project")
                                .index(1),
                        )
                        .arg(Arg::with_name("unset").long("unset").help("Unset private directory path"))
                        .group(ArgGroup::with_name("action").args(&["env_dir","unset"]).required(true))
                        .arg(setup_arg.clone()),
                ),
        )
        .subcommand(SubCommand::with_name("show").about("Show your current set up"))
        .subcommand(
            SubCommand::with_name("use")
                .about("Switch of current setup or/and environment")
                .arg(
                    Arg::with_name("setup")
                        .help("The setup name to switch in")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("environment")
                        .help("The environment name to switch in")
                        .index(2)
                ),
        )
        .subcommand(SubCommand::with_name("ls")
            .arg(setup_arg.clone())
            .arg(environment_arg.clone())
            .about("List set up and environments")
        )
        .get_matches();

    if let Some(_) = app.subcommand_matches("init") {
        commands::init(&app)?;
    } else if let Some(args) = app.subcommand_matches("new") {
        commands::new(&args)?;
    } else if let Some(args) = app.subcommand_matches("run") {
        commands::run(&args)?;
    } else if let Some(args) = app.subcommand_matches("ls") {
        commands::ls(&args)?;
    } else if let Some(args) = app.subcommand_matches("rename") {
        commands::rename(&args)?;
    } else if let Some(args) = app.subcommand_matches("use") {
        commands::r#use(&args)?;
    } else if let Some(_) = app.subcommand_matches("show") {
        commands::show()?;
    } else if let Some(args) = app.subcommand_matches("env") {
        if let Some(args) = args.subcommand_matches("new") {
            commands::env_new(args)?;
        } else if let Some(args) = args.subcommand_matches("edit") {
            commands::env_edit(args)?;
        } else if let Some(args) = args.subcommand_matches("dir") {
            commands::env_dir(args)?;
        } else if let Some(args) = args.subcommand_matches("pdir") {
            commands::env_pdir(args)?;
        }
    }

    Ok(())
}
