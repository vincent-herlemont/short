use predicates::prelude::Predicate;
use predicates::str::contains;

use test_utils::{PROJECT_CFG_FILE, PROJECT_ENV_DIR};
use test_utils::init;

mod test_utils;

#[test]
fn cmd_dir() {
    let mut e = init("cmd_env_dir");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  - name: setup_1
    file: run.sh
        "#,
    );
    e.add_dir(PROJECT_ENV_DIR);
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("dir")
        .arg("dir_not_found")
        .args(vec!["-s", "setup_1"])
        .assert()
        .failure()
        .to_string();

    assert!(contains("not found for `setup_1`").eval(&r));

    // SET

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("dir")
        .arg("env")
        .args(vec!["-s", "setup_1"])
        .assert()
        .success()
        .to_string();

    assert!(contains("env directory set to").eval(&r));

    let r = e.read_file(PROJECT_CFG_FILE);
    assert!(contains("public_env_dir: env").count(1).eval(&r));

    // UNSET

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("dir")
        .arg("--unset")
        .args(vec!["-s", "setup_1"])
        .assert()
        .success()
        .to_string();

    assert!(contains("env directory unset").eval(&r));

    let r = e.read_file(PROJECT_CFG_FILE);
    assert!(!contains("public_env_dir: env").count(1).eval(&r));

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("dir")
        .arg("--unset")
        .args(vec!["-s", "setup_1"])
        .assert()
        .failure()
        .to_string();

    assert!(contains("public env dir already unset for `setup_1`").eval(&r));
}
