use predicates::prelude::Predicate;
use predicates::str::contains;

use test_utils::init;

use crate::test_utils::{HOME_CFG_FILE, PROJECT_CFG_FILE, PROJECT_DIR};

mod test_utils;

#[test]
fn generate_template() {
    let mut e = init("generate_template");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: {}
        "#,
    );
    e.setup();
    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("generate")
        .arg("test_setup_1")
        .args(&["-t", "test"])
        .assert()
        .to_string();

    assert!(contains("generate setup `test_setup_1`:`dev`").eval(&r));

    let env_dev = e.path().unwrap().join(PROJECT_DIR).join("env/.dev");
    assert!(env_dev.exists());
    let run_sh = e.path().unwrap().join(PROJECT_DIR).join("run.sh");
    assert!(run_sh.exists());

    let r = e.read_file(PROJECT_CFG_FILE);
    assert_eq!(
        r#"---
setups:
  test_setup_1:
    public_env_dir: "./env/"
    file: run.sh
    array_vars:
      all: ".*"
    vars:
      - SETUP_NAME"#,
        &r
    );

    let r = e.read_file(HOME_CFG_FILE);
    assert!(contains("setup: test_setup_1").count(1).eval(&r));
    assert!(contains("env: dev").count(1).eval(&r));
    assert!(contains("test_setup_1").count(2).eval(&r));
}

#[test]
fn generate_template_with_target_directory() {
    let mut e = init("generate_template");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: {}
        "#,
    );
    e.setup();
    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("generate")
        .arg("test_setup_1")
        .args(&["-t", "test"])
        .args(&["-d", "target_directory"])
        .assert()
        .to_string();

    assert!(contains("generate setup `test_setup_1`:`dev`").eval(&r));

    let r = e.read_file(PROJECT_CFG_FILE);
    assert_eq!(
        r#"---
setups:
  test_setup_1:
    public_env_dir: target_directory/./env/
    file: target_directory/run.sh
    array_vars:
      all: ".*"
    vars:
      - SETUP_NAME"#,
        &r
    );
}

#[test]
fn generate_template_with_auto_target_directory() {
    let mut e = init("generate_template");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: {}
        "#,
    );
    e.setup();
    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("generate")
        .arg("test_setup_1")
        .args(&["-t", "test"])
        .args(&["-d"])
        .assert()
        .to_string();

    assert!(contains("generate setup `test_setup_1`:`dev`").eval(&r));

    let r = e.read_file(PROJECT_CFG_FILE);
    assert_eq!(
        r#"---
setups:
  test_setup_1:
    public_env_dir: test_setup_1/./env/
    file: test_setup_1/run.sh
    array_vars:
      all: ".*"
    vars:
      - SETUP_NAME"#,
        &r
    );
}
