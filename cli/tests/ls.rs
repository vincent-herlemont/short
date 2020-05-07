#[macro_use]
extern crate short_utils;
use predicates::prelude::Predicate;
use predicates::str;
use short_utils::integration_test::environment::IntegrationTestEnvironment;

#[test]
fn cmd_ls_settings() {
    let e = IntegrationTestEnvironment::new("cmd_help");
    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .assert()
        .to_string();
    let isSetup = str::is_match(r"setup None\n").unwrap();
    assert_eq!(isSetup.eval(r.as_str()), true);
    let isEnv = str::is_match(r"env None\n").unwrap();
    assert_eq!(isEnv.eval(r.as_str()), true);
}

#[test]
fn cmd_ls() {
    let e = IntegrationTestEnvironment::new("cmd_help");
    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .assert()
        .to_string();
    println!("{}", r);
}
