use clap::AppSettings::ArgRequiredElseHelp;
use clap::{App, Arg};
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
        .bin_name(BIN_NAME)
        .version(VERSION)
        .subcommand(App::new("watch").about("watch cloudformation infrastructure"))
        .subcommand(App::new("status").about("display of cloud formation infrastructure"))
        .arg(Arg::with_name("add").multiple(true))
        .subcommand(App::new("init"))
        .get_matches();

    if let Some(_) = app.subcommand_matches("init") {
        match init() {
            Ok(_) => {
                println!();
            }
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        }
    }
    if let Some(args) = app.values_of_lossy("add") {
        match add(args) {
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

fn add(args: Vec<String>) -> Result<()> {
    if let (Some(project_name), Some(path_to_yaml)) = (args.get(1), args.get(2)) {
        let mut projects = init()?;
        projects.add(project_name, path_to_yaml)?;
        println!("project name : {} ", project_name);
        println!("path to template : {}", path_to_yaml);
    } else {
        return Err(Error::from(
            "incorrect arguments : project name or path to yaml is missing",
        ));
    }
    Ok(())
}

fn init() -> Result<Projects> {
    match (current_dir(), dirs::home_dir()) {
        (Ok(current_dir), Some(home_dir)) => Projects::init(current_dir, home_dir),
        (Err(err), _) => Err(Error::wrap("init", Error::from(err))),
        (_, None) => Err(Error::from("fail to found your home directory")),
    }
}
