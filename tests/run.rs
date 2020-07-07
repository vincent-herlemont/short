use predicates::prelude::Predicate;
use predicates::str::contains;

use short::BIN_NAME;
use test_utils::init;
use test_utils::{PROJECT_CFG_FILE, PROJECT_ENV_EXAMPLE_1_FILE, PROJECT_RUN_FILE};

mod test_utils;

#[test]
fn cmd_run() {
    let mut e = init("cmd_run");

    e.add_file(PROJECT_ENV_EXAMPLE_1_FILE, r#"VAR1=VALUE1"#);
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  setup_1:
    file: run.sh
    array_vars:
      ALL: .*
    vars: [ VAR1 ]"#,
    );
    e.add_file(
        PROJECT_RUN_FILE,
        r#"#!/bin/bash
echo "TEST VAR1=$VAR1"
declare -p ALL
echo "ENVIRONMENT VAR $SHORT_ENV"
echo "SETUP VAR $SHORT_SETUP"
"#,
    );
    e.setup();
    e.set_exec_permission(PROJECT_RUN_FILE).unwrap();

    let mut command = e.command(BIN_NAME).unwrap();
    let command = command
        .env("RUST_LOG", "debug")
        .arg("run")
        .args(&vec!["-s", "setup_1"])
        .args(&vec!["-e", "example1"]);
    let r = command.assert().success().to_string();
    assert!(contains("#> TEST VAR1=VALUE1").count(1).eval(&r));
    assert!(contains("#> declare -x ALL=\"[VAR1]='VALUE1'\"")
        .count(1)
        .eval(&r));
    assert!(contains("ENVIRONMENT VAR example1").count(1).eval(&r));
    assert!(contains("SETUP VAR setup_1").count(1).eval(&r));
}

#[test]
fn cmd_run_with_args() {
    let mut e = init("cmd_run");

    e.add_file(PROJECT_ENV_EXAMPLE_1_FILE, r#"VAR1=VALUE1"#);
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  setup_1:
    file: run.sh
    array_vars:
      ALL: .*
    vars: [ VAR1 ]"#,
    );
    e.add_file(
        PROJECT_RUN_FILE,
        r#"#!/bin/bash
echo "TEST VAR1=$VAR1"
declare -p ALL
echo "ARG1 = $1"
echo "ARG2 = $2"
echo "ENVIRONMENT VAR $SHORT_ENV"
echo "SETUP VAR $SHORT_SETUP"
"#,
    );
    e.setup();
    e.set_exec_permission(PROJECT_RUN_FILE).unwrap();

    let mut command = e.command(BIN_NAME).unwrap();
    let command = command
        .env("RUST_LOG", "debug")
        .arg("run")
        .args(&["TEST_ARG1", "TEST_ARG2"])
        .args(&vec!["-s", "setup_1"])
        .args(&vec!["-e", "example1"]);
    let r = command.assert().success().to_string();
    assert!(contains("#> TEST VAR1=VALUE1").count(1).eval(&r));
    assert!(contains("#> declare -x ALL=\"[VAR1]='VALUE1'\"")
        .count(1)
        .eval(&r));
    assert!(contains("ENVIRONMENT VAR example1").count(1).eval(&r));
    assert!(contains("ARG1 = TEST_ARG1").count(1).eval(&r));
    assert!(contains("ARG2 = TEST_ARG2").count(1).eval(&r));
    assert!(contains("SETUP VAR setup_1").count(1).eval(&r));
}
