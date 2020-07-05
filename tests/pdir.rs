use predicates::prelude::Predicate;
use predicates::str::contains;

use test_utils::init;
use test_utils::{HOME_CFG_FILE, PRIVATE_ENV_DIR, PROJECT_CFG_FILE};

mod test_utils;

#[test]
fn cmd_pdir() {
    let mut e = init("cmd_env_pdir");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  setup_1:
    file: run.sh
        "#,
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("pdir")
        .arg(format!("../dir_not_found"))
        .args(vec!["-s", "setup_1"])
        .assert()
        .failure()
        .to_string();

    contains("not found for `setup_1`").eval(&r);

    // SET

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("pdir")
        .arg(format!("../{}", PRIVATE_ENV_DIR))
        .args(vec!["-s", "setup_1"])
        .assert()
        .success()
        .to_string();

    assert!(contains("private env directory set to").eval(&r));

    let r = e.read_file(HOME_CFG_FILE);
    assert!(contains("private_env_dir:").count(1).eval(&r));

    // UNSET

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("pdir")
        .arg("--unset")
        .args(vec!["-s", "setup_1"])
        .assert()
        .success()
        .to_string();

    assert!(contains("private env directory unset").eval(&r));

    let r = e.read_file(HOME_CFG_FILE);
    assert!(!contains("private_env_dir:").count(1).eval(&r));

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("pdir")
        .arg("--unset")
        .args(vec!["-s", "setup_1"])
        .assert()
        .failure()
        .to_string();

    assert!(contains("private env dir already unset for `setup_1`").eval(&r));
}
