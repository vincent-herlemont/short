use predicates::prelude::Predicate;
use predicates::str::contains;

use test_utils::init;

use crate::test_utils::{
    HOME_CFG_FILE, PROJECT_CFG_FILE, PROJECT_ENV_EXAMPLE_1_FILE, PROJECT_RUN_FILE,
};
use short::BIN_NAME;

mod test_utils;

#[test]
fn cmd_generate() {
    let mut e = init("cmd_generate");

    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: {}
"#,
    );
    e.setup();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("generate")
        .arg("setup_1")
        .arg("example1")
        .assert()
        .success()
        .to_string();

    assert!(contains("generate setup").eval(&r));
    debug_assert!(e.file_exists(PROJECT_ENV_EXAMPLE_1_FILE));

    let r = e.read_file(PROJECT_CFG_FILE);
    assert!(contains("setup_1").eval(&r));

    let r = e.read_file(PROJECT_RUN_FILE);
    assert!(contains("declare -A all && eval all=($ALL)").eval(&r));

    let r = e.read_file(HOME_CFG_FILE);
    assert!(contains("current").count(1).eval(&r));
    assert!(contains("setup: setup_1").count(1).eval(&r));
    assert!(contains("env: example").count(1).eval(&r));
}

#[test]
fn cmd_generate_with_existing_env() {
    let mut e = init("cmd_generate_with_existing_env");
    let local_env_example_content = r#"VAR1=VALUE1"#;
    e.add_file(PROJECT_ENV_EXAMPLE_1_FILE, local_env_example_content);
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: {}
    "#,
    );
    e.setup();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("generate")
        .arg("setup_1")
        .arg("example")
        .assert()
        .success()
        .to_string();

    assert!(contains("generate setup").eval(&r));

    assert!(e.file_exists(PROJECT_ENV_EXAMPLE_1_FILE));
    let r = e.read_file(PROJECT_ENV_EXAMPLE_1_FILE);
    assert_eq!(r, local_env_example_content);

    let r = e.read_file(PROJECT_CFG_FILE);
    assert!(contains("setup_1").eval(&r));

    let r = e.read_file(PROJECT_RUN_FILE);
    assert!(contains("declare -A all && eval all=($ALL)").eval(&r));

    let r = e.read_file(HOME_CFG_FILE);
    assert!(contains("current").count(1).eval(&r));
    assert!(contains("setup: setup_1").count(1).eval(&r));
    assert!(contains("env: example").count(1).eval(&r));
}

const PROJECT_OTHER_RUN_FILE: &'static str = "project/other_run/run.sh";

#[test]
fn cmd_generate_with_file_sub_directory() {
    let mut e = init("cmd_generate_with_target_directory");
    e.add_file(
        PROJECT_OTHER_RUN_FILE,
        r#"#!/bin/sh
echo "TEST"
"#,
    );
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: {}
    "#,
    );
    e.setup();
    e.set_exec_permission(PROJECT_OTHER_RUN_FILE).unwrap();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("generate")
        .arg("setup_1")
        .arg("example1")
        .args(&["-f", "other_run/run.sh"])
        .assert()
        .failure()
        .to_string();

    assert!(contains("file `\"other_run/run.sh\"` already exists").eval(&r));
}

#[test]
fn cmd_generate_with_file_sub_directory_not_found() {
    let mut e = init("cmd_generate_with_file_sub_directory_not_found");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: {}
    "#,
    );
    e.setup();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("generate")
        .arg("setup_1")
        .arg("example1")
        .args(&["-f", "other_run/run.sh"])
        .assert()
        .to_string();

    assert!(contains("generate setup").eval(&r));

    assert!(e.file_exists(PROJECT_ENV_EXAMPLE_1_FILE));

    let r = e.read_file(PROJECT_CFG_FILE);
    assert!(contains("setup_1").eval(&r));

    let r = e.read_file(PROJECT_OTHER_RUN_FILE);
    assert!(contains("declare -A all && eval all=($ALL)").eval(&r));

    let r = e.read_file(HOME_CFG_FILE);
    assert!(contains("current").count(1).eval(&r));
    assert!(contains("setup: setup_1").count(1).eval(&r));
    assert!(contains("env: example").count(1).eval(&r));
}

#[test]
fn cmd_generate_with_file_env_directory() {
    let mut e = init("cmd_generate_with_file_env_directory");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: {}
    "#,
    );
    e.setup();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("generate")
        .arg("setup_1")
        .arg("example1")
        .args(&["-e", "public_env"])
        .assert()
        .to_string();

    assert!(contains("generate setup").eval(&r));

    assert!(e.file_exists("project/public_env/.example1"));
}
