use insta::assert_debug_snapshot;
use std::fs::read_to_string;
use std::io::{BufRead, BufReader, Read};
use std::process::Stdio;
use utils::asset::Assets;
use utils::test::{before, ConfigPath};

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn demo() {
    // DEMO indicatif
    let config = before("add", Assets::None).cli(CRATE_NAME);
    let mut command = config.command();
    command.arg("demo");
    //let output = command.output().unwrap();
}
