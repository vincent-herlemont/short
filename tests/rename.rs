use predicates::prelude::Predicate;
use predicates::str::contains;

use test_utils::init;
use test_utils::{HOME_CFG_FILE, PROJECT_CFG_FILE};

mod test_utils;

#[test]
fn cmd_rename() {
    let mut e = init("cmd_rename");

    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  - name: setup_1
    file: run.sh
    array_vars: {}
        "#,
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("rename")
        .arg("setup_1")
        .arg("setup_2")
        .assert()
        .success()
        .to_string();

    assert!(contains("setup renamed").eval(&r));

    let r = e.read_file(PROJECT_CFG_FILE);
    assert!(!contains("setup_1").eval(&r));
    assert!(contains("setup_2").eval(&r));

    let r = e.read_file(HOME_CFG_FILE);
    assert!(!contains("setup_1").eval(&r));
    assert!(contains("setup_2").count(1).eval(&r));
}

#[test]
fn cmd_rename_with_use() {
    let mut e = init("cmd_rename_with_use");

    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  - name: setup_1
    file: run.sh
    array_vars: {}
        "#,
    );
    e.add_file(
        HOME_CFG_FILE,
        format!(
            r#"---
projects:
  - file: {file}
    current:
      setup: setup_1
    setups:
      - name: setup_1
            "#,
            file = e.path().join(PROJECT_CFG_FILE).to_string_lossy()
        ),
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("rename")
        .arg("setup_1")
        .arg("setup_2")
        .assert()
        .success()
        .to_string();

    assert!(contains("setup renamed").eval(&r));

    let r = e.read_file(PROJECT_CFG_FILE);
    assert!(!contains("setup_1").eval(&r));
    assert!(contains("setup_2").eval(&r));

    let r = e.read_file(HOME_CFG_FILE);
    assert!(!contains("setup_1").eval(&r));
    assert!(contains("setup_2").count(2).eval(&r));
}
