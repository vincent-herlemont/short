use clap::AppSettings::{ArgRequiredElseHelp, VersionlessSubcommands};
use clap::{App, Arg, ArgMatches};

use d4d::exec::ExecCtx;

use crate::command::{add_command, deploy_command, env_command, init_command, use_command};
use crate::init::{init_exec_ctx, init_projects};
use d4d::project::Projects;
use std::env;

use std::process::exit;

use utils::result::Result;

mod assets;
mod command;
mod helper;
mod init;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const BIN_NAME: &'static str = "d4d";

fn main() {
    let app = App::new(BIN_NAME)
        .setting(ArgRequiredElseHelp)
        .setting(VersionlessSubcommands)
        .bin_name(BIN_NAME)
        .version(VERSION)
        .about("Cloud environment deployment")
        .arg(
            Arg::with_name("project")
                .long("project")
                .short("p")
                .takes_value(true)
                .global(true)
                .help("Project name"),
        )
        .arg(
            Arg::with_name("env")
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
            App::new("add")
                .about("Add new project")
                .arg(Arg::with_name("add_project").multiple(true)),
        )
        .subcommand(
            App::new("use")
                .about("Set an current project and environment")
                .arg(Arg::with_name("use_project").multiple(true)),
        )
        .subcommand(App::new("deploy").about("Deploy on the cloud"))
        .subcommand(App::new("init").about("Create en empty configuration file"))
        .subcommand(
            App::new("env")
                .setting(ArgRequiredElseHelp)
                .about("Manage environment files")
                .arg(
                    Arg::with_name("check")
                        .help("Verified env coherence and syntax")
                        .long("check")
                        .short("c"),
                ),
        )
        .get_matches();

    // Match init command first in order to allow him to create project file
    // before any other execution.
    if let Some(_args) = app.subcommand_matches("init") {
        match init_command() {
            Ok(()) => println!(),
            Err(err) => eprintln!("{}", err),
        }
        return;
    }

    // Load execution context
    let exec_ctx = init_exec_ctx(&app);

    // Launch commands
    match init_projects(&app) {
        Ok(projects) => match dispatch_commands(exec_ctx, projects, app) {
            Ok(()) => println!(),
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        },
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

fn dispatch_commands(exec_ctx: ExecCtx, mut projects: Projects, app: ArgMatches) -> Result<()> {
    if let Some(args) = app.subcommand_matches("env") {
        return env_command(&args, &projects);
    } else if let Some(args) = app.subcommand_matches("add") {
        return add_command(args, &mut projects);
    } else if let Some(args) = app.subcommand_matches("use") {
        return use_command(args, &mut projects);
    } else if let Some(_) = app.subcommand_matches("deploy") {
        return deploy_command(&exec_ctx, &projects);
    }
    Ok(())
}
