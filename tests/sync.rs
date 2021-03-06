use std::thread;
use std::time::Duration;

use predicates::prelude::Predicate;
use predicates::str::contains;

use short::BIN_NAME;
use test_utils::init;
use test_utils::{PROJECT_CFG_FILE, PROJECT_ENV_EXAMPLE_1_FILE, PROJECT_ENV_EXAMPLE_2_FILE};

mod test_utils;

#[test]
fn cmd_sync_add_empty() {
    let mut e = init("cmd_env_sync_add_empty");
    e.add_file(
        PROJECT_ENV_EXAMPLE_1_FILE,
        r#"VAR1=VALUE1
"#,
    );
    e.add_file(
        PROJECT_ENV_EXAMPLE_2_FILE,
        r#"VAR1=VALUE1
VAR2=VALUE2
"#,
    );
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  setup_1:
    file: run.sh
        "#,
    );
    e.setup();
    thread::sleep(Duration::from_secs(2));
    e.set_update_file_time(PROJECT_ENV_EXAMPLE_2_FILE).unwrap();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("sync")
        .arg("--empty")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("files synchronized").eval(&r));

    let target = e.read_file(PROJECT_ENV_EXAMPLE_1_FILE);
    let source = e.read_file(PROJECT_ENV_EXAMPLE_2_FILE);
    assert_eq!(
        target,
        r#"VAR1=VALUE1
VAR2=
"#
    );
    assert_eq!(
        source,
        r#"VAR1=VALUE1
VAR2=VALUE2
"#
    )
}

#[test]
fn cmd_sync_add_empty_source_file() {
    let mut e = init("cmd_env_sync_add_empty");
    e.add_file(
        PROJECT_ENV_EXAMPLE_1_FILE,
        r#"VAR1=VALUE1
VAR2=VALUE2
"#,
    );
    e.add_file(
        PROJECT_ENV_EXAMPLE_2_FILE,
        r#"VAR1=VALUE1
"#,
    );
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  setup_1:
    file: run.sh
        "#,
    );
    e.setup();
    thread::sleep(Duration::from_secs(2));
    e.set_update_file_time(PROJECT_ENV_EXAMPLE_2_FILE).unwrap();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("sync")
        .arg("--empty")
        .args(&["-f", ".example1"])
        .args(&["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("files synchronized").eval(&r));

    let source = e.read_file(PROJECT_ENV_EXAMPLE_1_FILE);
    let target = e.read_file(PROJECT_ENV_EXAMPLE_2_FILE);
    assert_eq!(
        target,
        r#"VAR1=VALUE1
VAR2=
"#
    );
    assert_eq!(
        source,
        r#"VAR1=VALUE1
VAR2=VALUE2
"#
    )
}

#[test]
fn cmd_sync_add_copy() {
    let mut e = init("cmd_env_sync_add_copy");
    e.add_file(
        PROJECT_ENV_EXAMPLE_1_FILE,
        r#"VAR1=VALUE1
"#,
    );
    e.add_file(
        PROJECT_ENV_EXAMPLE_2_FILE,
        r#"VAR1=VALUE1
VAR2=VALUE2
"#,
    );
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  setup_1:
    file: run.sh
        "#,
    );

    e.setup();
    thread::sleep(Duration::from_secs(2));
    e.set_update_file_time(PROJECT_ENV_EXAMPLE_2_FILE).unwrap();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("sync")
        .arg("--copy")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("files synchronized").eval(&r));

    let target = e.read_file(PROJECT_ENV_EXAMPLE_1_FILE);
    let source = e.read_file(PROJECT_ENV_EXAMPLE_2_FILE);
    assert_eq!(target, source);
}

#[test]
fn cmd_sync_delete() {
    let mut e = init("cmd_env_sync_delete");
    e.add_file(
        PROJECT_ENV_EXAMPLE_1_FILE,
        r#"VAR1=VALUE1
VAR3=VALUE3
"#,
    );
    e.add_file(
        PROJECT_ENV_EXAMPLE_2_FILE,
        r#"VAR1=VALUE1
"#,
    );
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  setup_1:
    file: run.sh
        "#,
    );

    e.setup();
    thread::sleep(Duration::from_secs(2));
    e.set_update_file_time(PROJECT_ENV_EXAMPLE_2_FILE).unwrap();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("sync")
        .arg("--delete")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("files synchronized").eval(&r));

    let target = e.read_file(PROJECT_ENV_EXAMPLE_1_FILE);
    let source = e.read_file(PROJECT_ENV_EXAMPLE_2_FILE);
    assert_eq!(target, source);
}

#[test]
fn cmd_sync_no_delete() {
    let mut e = init("cmd_env_sync_no_delete");
    let initial_target = r#"VAR1=VALUE1
VAR3=VALUE3
"#;
    let inital_source = r#"VAR1=VALUE1
"#;

    e.add_file(PROJECT_ENV_EXAMPLE_1_FILE, &initial_target);
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  setup_1:
    file: run.sh
        "#,
    );
    e.add_file(PROJECT_ENV_EXAMPLE_2_FILE, &inital_source);
    e.setup();
    thread::sleep(Duration::from_secs(2));
    e.set_update_file_time(PROJECT_ENV_EXAMPLE_2_FILE).unwrap();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("sync")
        .arg("--no_delete")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("you have not allowed to delete var `VAR3`:`VALUE3` in example").eval(&r));
    assert!(
        contains("env must be sync, please change it manually or run \"short env sync\"").eval(&r)
    );

    let target = e.read_file(PROJECT_ENV_EXAMPLE_1_FILE);
    let source = e.read_file(PROJECT_ENV_EXAMPLE_2_FILE);
    assert_eq!(target, initial_target);
    assert_eq!(source, inital_source);
}
