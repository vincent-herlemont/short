use predicates::prelude::Predicate;
use predicates::str::contains;
use std::path::PathBuf;
use utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment, PROJECT};

mod utils;

#[test]
fn cmd_env_dir() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_use");
    let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
    {
        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(
            &local_cfg_file,
            r#"
setups:
  - name: setup_1
    file: run.sh
        "#,
        );
        e.add_dir(PathBuf::from(PROJECT).join("env"));
        e.setup();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("dir")
        .arg("dir_not_found")
        .args(vec!["-s", "setup_1"])
        .assert()
        .failure()
        .to_string();

    assert!(contains("not found for `setup_1`").eval(&r));

    // SET

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("dir")
        .arg("env")
        .args(vec!["-s", "setup_1"])
        .assert()
        .success()
        .to_string();

    assert!(contains("env directory set to").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow();
        let r = e.read_file(&local_cfg_file);
        assert!(contains("public_env_dir: env").count(1).eval(&r));
    }

    // UNSET

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("dir")
        .arg("--unset")
        .args(vec!["-s", "setup_1"])
        .assert()
        .success()
        .to_string();

    assert!(contains("env directory unset").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow();
        let r = e.read_file(&local_cfg_file);
        assert!(!contains("public_env_dir: env").count(1).eval(&r));
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("dir")
        .arg("--unset")
        .args(vec!["-s", "setup_1"])
        .assert()
        .failure()
        .to_string();

    assert!(contains("public env dir already unset for `setup_1`").eval(&r));
}
