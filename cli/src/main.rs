use clap::AppSettings::{ArgRequiredElseHelp, VersionlessSubcommands};
use clap::{App, Arg, ArgMatches};
use d4d::env as d4denv;
use d4d::project::Projects;
use std::env;
use std::env::current_dir;
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
        .arg(
            Arg::with_name("project")
                .long("project")
                .short("p")
                .takes_value(true)
                .global(true),
        )
        .arg(
            Arg::with_name("env")
                .long("env")
                .short("e")
                .takes_value(true)
                .global(true),
        )
        .subcommand(App::new("watch").about("watch cloudformation infrastructure"))
        .subcommand(App::new("status").about("display of cloud formation infrastructure"))
        .subcommand(App::new("add").arg(Arg::with_name("add_project").multiple(true)))
        .subcommand(App::new("use").arg(Arg::with_name("use_project").multiple(true)))
        .subcommand(App::new("init"))
        .subcommand(
            App::new("env").setting(ArgRequiredElseHelp).arg(
                Arg::with_name("check")
                    .help("Verified env syntax and coherence")
                    .long("check")
                    .short("c"),
            ),
        )
        .get_matches();

    match init_projects() {
        Ok(projects) => init(projects, app),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

fn init(mut projects: Projects, app: ArgMatches) {
    if let Some(_) = app.subcommand_matches("init") {
        println!();
    } else if let Some(args) = app.subcommand_matches("env") {
        match env(&args) {
            Ok(_) => {
                println!();
            }
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        }
    } else if let Some(args) = app.subcommand_matches("add") {
        match add(args, &mut projects) {
            Ok(_) => {
                println!();
            }
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        }
    } else if let Some(args) = app.subcommand_matches("use") {
        match r#use(args, &mut projects) {
            Ok(_) => {
                println!();
            }
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        }
    }
}

fn r#use(args: &ArgMatches, projects: &mut Projects) -> Result<()> {
    if let Some(args) = args.values_of_lossy("use_project") {
        if let (Some(project_name), Some(env_name)) = (args.get(0), args.get(1)) {
            dbg!(project_name, env_name);
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

fn init_projects() -> Result<Projects> {
    match (current_dir(), dirs::home_dir()) {
        (Ok(current_dir), Some(home_dir)) => Projects::init(current_dir, home_dir),
        (Err(err), _) => Err(Error::wrap("init", Error::from(err))),
        (_, None) => Err(Error::from("fail to found your home directory")),
    }
}

fn env(args: &ArgMatches) -> Result<()> {
    if let (Some(_), Some(vp), Some(ve)) = (
        args.values_of_lossy("check"),
        args.values_of_lossy("project"),
        args.values_of_lossy("env"),
    ) {
        if let (Some(project), Some(env)) = (vp.first(), ve.first()) {
            let projects = init_projects().unwrap();
            if let Ok(project) = projects.found(project) {
                let env = d4denv::get(&project, &env)?;
                println!("{}", &project);
                print!("{}", &env);
                return Ok(());
            } else {
                return Err(Error::new(format!("fail to found project {}", project)));
            }
        } else {
            return Err(Error::new("fail to get env or project"));
        }
    }
    Ok(())
}
