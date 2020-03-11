use clap::App;
use clap::AppSettings::ArgRequiredElseHelp;
use d4d::project::Projects;
use std::env;
use std::env::current_dir;
use std::process::exit;
use utils::result::Result;

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
        match init() {
            Ok(_) => {
                println!("");
            }
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        }
    }
}

fn init() -> Result<()> {
    match (current_dir(), dirs::home_dir()) {
        (Ok(current_dir), Some(home_dir)) => {
            Projects::init(current_dir, home_dir)?;
        }
        (Err(err), _) => {
            eprintln!("{}", err);
        }
        (_, None) => {
            eprintln!("fail to found your home directory");
        }
    }
    Ok(())
}
