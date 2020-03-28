use clap::AppSettings::{ArgRequiredElseHelp, VersionlessSubcommands};
use clap::{App, Arg, ArgMatches};
use d4d::env as d4denv;
use d4d::env::get;
use d4d::exec::aws::workflow::AwsWorkflow;
use d4d::exec::ExecCtx;

use d4d::project::{CurrentProject, Projects};
use std::env;
use std::env::current_dir;
use std::path::PathBuf;
use std::process::exit;
use utils::error::Error;
use utils::result::Result;

mod assets;

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

    if let Some(_args) = app.subcommand_matches("init") {
        match init_command() {
            Ok(()) => println!(),
            Err(err) => eprintln!("{}", err),
        }
        return;
    }

    let exec_ctx = init_exec_ctx(&app);

    match init_projects(&app) {
        Ok(projects) => match dispatch_core_commands(exec_ctx, projects, app) {
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

fn init_command() -> Result<()> {
    match reach_directories() {
        Ok((curent_dir, _)) => {
            Projects::init(&curent_dir)?;
            Ok(())
        }
        Err(err) => Err(Error::wrap("fail to init project", Error::from(err))),
    }
}

fn init_exec_ctx(app: &ArgMatches) -> ExecCtx {
    let exec_ctx = ExecCtx::new();
    if app.is_present("dry-run") {
        exec_ctx.set_dry_run(true)
    } else {
        exec_ctx
    }
}

fn dispatch_core_commands(
    exec_ctx: ExecCtx,
    mut projects: Projects,
    app: ArgMatches,
) -> Result<()> {
    if let Some(args) = app.subcommand_matches("env") {
        return env(&args, &projects);
    } else if let Some(args) = app.subcommand_matches("add") {
        return add(args, &mut projects);
    } else if let Some(args) = app.subcommand_matches("use") {
        return r#use(args, &mut projects);
    } else if let Some(_) = app.subcommand_matches("deploy") {
        return deploy(&exec_ctx, &projects);
    }
    Ok(())
}

fn r#use(args: &ArgMatches, projects: &mut Projects) -> Result<()> {
    if let Some(args) = args.values_of_lossy("use_project") {
        if let (Some(project_name), Some(env_name)) = (args.get(0), args.get(1)) {
            projects.set_current_project_name(project_name);
            projects.set_current_env_name(env_name)?;
            projects.save()?;
        } else {
            return Err(Error::from(
                "incorrect arguments : project name or env name is missing",
            ));
        }
    }
    Ok(())
}

fn add(args: &ArgMatches, projects: &mut Projects) -> Result<()> {
    if let Some(args) = args.values_of_lossy("add_project") {
        if let (Some(project_name), Some(path_to_yaml)) = (args.get(0), args.get(1)) {
            projects.add(project_name, path_to_yaml)?;
            println!("project name : {} ", project_name);
            println!("path to template : {}", path_to_yaml);
        } else {
            return Err(Error::from(
                "incorrect arguments : project name or path to yaml is missing",
            ));
        }
    }
    Ok(())
}

fn init_projects(args: &ArgMatches) -> Result<Projects> {
    match (current_dir(), dirs::home_dir()) {
        (Ok(current_dir), Some(home_dir)) => {
            let mut projects = Projects::load(current_dir, home_dir)?;
            if let Some(project_name) = args.value_of_lossy("project") {
                let current_project = CurrentProject::new(project_name);
                projects.set_temporary_current_project(current_project.clone());
                if let Some(env) = args.value_of_lossy("env") {
                    let current_project = current_project.set_env(env);
                    projects.set_temporary_current_project(current_project);
                }
            }
            Ok(projects)
        }
        (Err(err), _) => Err(Error::wrap("init", Error::from(err))),
        (_, None) => Err(Error::from("fail to found your home directory")),
    }
}

/// return (current_dir,home_dir)
fn reach_directories() -> Result<(PathBuf, PathBuf)> {
    match (current_dir(), dirs::home_dir()) {
        (Ok(current_dir), Some(home_dir)) => Ok((current_dir, home_dir)),
        (Err(err), _) => Err(Error::wrap(
            "fail to reach current directory",
            Error::from(err),
        )),
        (_, None) => Err(Error::new(
            "fail to reach home directory, please check your $HOME (linux,osx) or FOLDERID_Profile (windows)",
        )),
    }
}

fn env(args: &ArgMatches, projects: &Projects) -> Result<()> {
    if let (Some(_), Some(vp), Some(ve)) = (
        args.values_of_lossy("check"),
        args.values_of_lossy("project"),
        args.values_of_lossy("env"),
    ) {
        if let (Some(project), Some(env)) = (vp.first(), ve.first()) {
            let project = projects.found(project)?;
            let env = d4denv::get(&project, &env)?;
            println!("{}", &project);
            print!("{}", &env);
            return Ok(());
        } else {
            return Err(Error::new("fail to get env or project"));
        }
    }
    Ok(())
}

fn deploy(exec_ctx: &ExecCtx, projects: &Projects) -> Result<()> {
    let project = projects.current_project()?;
    let env = projects.current_env()?;
    let env = get(&project, &env)?;

    println!("--dry-run");
    let runner = AwsWorkflow::new(&project, &exec_ctx)?.package(&env)?;
    println!("{}", runner);
    let runner = AwsWorkflow::new(&project, &exec_ctx)?.deploy(&env)?;
    println!("{}", runner);
    Ok(())
}
