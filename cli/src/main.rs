#[macro_use]
extern crate log;

use exitfailure::ExitFailure;
use std::env;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const BIN_NAME: &'static str = "short";

fn main() -> Result<(), ExitFailure> {
    env_logger::init();
    info!("BIN_NAME {}", BIN_NAME);
    info!("VERSION v{}", VERSION);
    Ok(run()?)
}

fn run() -> Result<(), failure::Error> {
    Ok(())
}
