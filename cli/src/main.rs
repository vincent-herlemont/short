#[macro_use]
extern crate log;
mod terminal;

use clap::{App, AppSettings, Arg, SubCommand};
use exitfailure::ExitFailure;
use std::env;
use terminal::emoji;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const BIN_NAME: &'static str = "short";

fn main() -> Result<(), ExitFailure> {
    env_logger::init();
    info!("BIN_NAME {}", BIN_NAME);
    info!("VERSION v{}", VERSION);
    Ok(run()?)
}

fn run() -> Result<(), failure::Error> {
    App::new(format!("{} short", emoji::PARASOL))
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
                                .long("n")
                                .takes_value(true),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("env")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Manage environment")
                .subcommand(
                    SubCommand::with_name("new").arg(
                        Arg::with_name("name")
                            .help("name of your environment")
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
    Ok(())
}
