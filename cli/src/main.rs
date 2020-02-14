use clap::App;
use clap::AppSettings::ArgRequiredElseHelp;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const BIN_NAME: &'static str = "d4d";

fn main() {
    App::new(BIN_NAME)
        .setting(ArgRequiredElseHelp)
        .bin_name(BIN_NAME)
        .version(VERSION)
        .subcommand(App::new("add").about("add preset configuration"))
        .subcommand(App::new("watch").about("watch cloudformation infrastructure"))
        .subcommand(App::new("status").about("display of cloud formation infrastructure"))
        .get_matches();
}
