use clap::App;
use clap::AppSettings::ArgRequiredElseHelp;
use d4d::project::local::LocalProjects;
use std::env;
use std::process::exit;

mod assets;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const BIN_NAME: &'static str = "d4d";

fn main() {
    let app = App::new(BIN_NAME)
        .setting(ArgRequiredElseHelp)
        .bin_name(BIN_NAME)
        .version(VERSION)
        .subcommand(App::new("add").about("add preset configuration"))
        .subcommand(App::new("watch").about("watch cloudformation infrastructure"))
        .subcommand(App::new("status").about("display of cloud formation infrastructure"))
        .subcommand(App::new("init"))
        .get_matches();

    if let Some(_) = app.subcommand_matches("init") {
        if let Some(home) = dirs::home_dir() {
            if let Ok(local_projects) = LocalProjects::new(home) {
                println!("{}", local_projects);
            } else {
                eprintln!("fail to read local projects");
            }
        } else {
            eprintln!("Fail to found the $HOME directories");
            exit(1);
        }
    }
}
