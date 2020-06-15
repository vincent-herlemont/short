use predicates::prelude::Predicate;
use predicates::str::contains;

use test_utils::init;
use test_utils::{
    PROJECT_CFG_FILE, PROJECT_ENV_EXAMPLE_1_FILE, PROJECT_ENV_EXAMPLE_2_FILE,
};

mod test_utils;

const MOCK_EDITOR_FILE: &'static str = "mock_editor.sh";

#[test]
fn cmd_env_edit() {
    let mut e = init("cmd_env_edit");

    e.add_file(PROJECT_ENV_EXAMPLE_1_FILE, r#"VAR1=VALUE1"#);
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  - name: setup_1
    file: run.sh
        "#,
    );
    e.add_file(
        MOCK_EDITOR_FILE,
        r#"#!/bin/bash
echo -e "\nVAR2=VALUE2" >> $1
        "#,
    );
    e.setup();
    e.set_exec_permission(MOCK_EDITOR_FILE).unwrap();

    let mock_editor_file_abs = e.path().join(MOCK_EDITOR_FILE);

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("edit")
        .arg("example1")
        .args(vec!["-s", "setup_1"])
        .args(vec![
            "--editor",
            mock_editor_file_abs.to_string_lossy().into_owned().as_str(),
        ])
        .assert()
        .to_string();

    assert!(contains("`example1` edited").count(1).eval(&r));

    let r = e.read_file(PROJECT_ENV_EXAMPLE_1_FILE);
    assert!(contains("VAR2=VALUE2").count(1).eval(&r));
}

#[test]
fn cmd_env_edit_with_sync() {
    let mut e = init("cmd_env_edit");
    e.add_file(PROJECT_ENV_EXAMPLE_1_FILE, r#"VAR1=VALUE1"#);
    e.add_file(PROJECT_ENV_EXAMPLE_2_FILE, r#"VAR1=VALUE1"#);
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  - name: setup_1
    file: run.sh
        "#,
    );

    e.add_file(
        MOCK_EDITOR_FILE,
        r#"#!/bin/bash
echo -e "\nVAR2=VALUE2" >> $1
        "#,
    );
    e.setup();
    e.set_exec_permission(MOCK_EDITOR_FILE).unwrap();

    let mock_editor_file_abs = e.path().join(MOCK_EDITOR_FILE);

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("edit")
        .arg("example1")
        .arg("--copy")
        .args(vec!["-s", "setup_1"])
        .args(vec![
            "--editor",
            mock_editor_file_abs.to_string_lossy().into_owned().as_str(),
        ])
        .assert()
        .to_string();

    assert!(contains("`example1` edited").count(1).eval(&r));

    let r = e.read_file(PROJECT_ENV_EXAMPLE_1_FILE);
    assert!(contains("VAR2=VALUE2").count(1).eval(&r));
    let r = e.read_file(PROJECT_ENV_EXAMPLE_2_FILE);
    assert!(contains("VAR2=VALUE2").count(1).eval(&r));
}
