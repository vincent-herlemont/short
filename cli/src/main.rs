#[macro_use]
extern crate prettytable;

use clap::AppSettings::{ArgRequiredElseHelp, DeriveDisplayOrder, VersionlessSubcommands};
use clap::{App, Arg, ArgMatches};

use short::exec::ExecCtx;

use crate::command::{
    add_command, demo_command, init_command, ls_command, run_command, use_command,
};
use crate::init::{init_exec_ctx, init_projects};
use short::project::Projects;
use std::env;

use std::process::exit;

use utils::result::Result;

mod assets;
mod command;
mod helper;
mod init;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const BIN_NAME: &'static str = "short";

fn main() {
    let app = App::new(BIN_NAME)
        .setting(ArgRequiredElseHelp)
        .setting(VersionlessSubcommands)
        .setting(DeriveDisplayOrder)
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
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .global(true)
                .help("Display all cammands"),
        )
        .subcommand(App::new("init").about("Create en empty configuration file"))
        .subcommand(
            App::new("add")
                .about("[project_name] [template_file] Add new project")
                .arg(Arg::with_name("add_project").multiple(true)),
        )
        .subcommand(
            App::new("use")
                .about("[project_name] [environment_name] Set an current project and environment")
                .arg(Arg::with_name("use_project").multiple(true)),
        )
        .subcommand(App::new("run").about("Run project"))
        .subcommand(App::new("ls").about("List projects"))
        .get_matches();

    // Display version
    if app.is_present("demo") {
        demo_command();
        return;
    }

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
    if let Some(args) = app.subcommand_matches("add") {
        return add_command(args, &mut projects);
    } else if let Some(args) = app.subcommand_matches("use") {
        return use_command(args, &mut projects);
    } else if let Some(_) = app.subcommand_matches("run") {
        return run_command(&exec_ctx, &projects);
    } else if let Some(_) = app.subcommand_matches("ls") {
        return ls_command(&projects);
    }
    Ok(())
}
