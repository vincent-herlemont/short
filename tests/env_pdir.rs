
use predicates::prelude::Predicate;
use predicates::str::contains;

use utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment, ENVDIR};

mod utils;

#[test]
fn cmd_env_pdir() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_use");
    let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
    {
        let global_env_dev_file = itew
            .get_abs_path(PathTestEnvironment::GlobalEnvDev)
            .unwrap();
        let _global_env_dir = global_env_dev_file.parent().unwrap();

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
        e.setup();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("pdir")
        .arg(format!("../dir_not_found"))
        .args(vec!["-s", "setup_1"])
        .assert()
        .failure()
        .to_string();

    contains("not found for `setup_1`").eval(&r);

    // SET

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("pdir")
        .arg(format!("../{}", ENVDIR))
        .args(vec!["-s", "setup_1"])
        .assert()
        .success()
        .to_string();

    assert!(contains("private env directory set to").eval(&r));
    {
        let e = itew.e();
        let e = e.borrow();
        let global_cfg_file = itew.get_rel_path(PathTestEnvironment::GlobalCfg).unwrap();
        let r = e.read_file(global_cfg_file);
        assert!(contains("private_env_dir:").count(1).eval(&r));
    }

    // UNSET

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("pdir")
        .arg("--unset")
        .args(vec!["-s", "setup_1"])
        .assert()
        .success()
        .to_string();

    assert!(contains("private env directory unset").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow();
        let global_cfg_file = itew.get_rel_path(PathTestEnvironment::GlobalCfg).unwrap();
        let r = e.read_file(global_cfg_file);
        assert!(!contains("private_env_dir:").count(1).eval(&r));
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("pdir")
        .arg("--unset")
        .args(vec!["-s", "setup_1"])
        .assert()
        .failure()
        .to_string();

    assert!(contains("private env dir already unset for `setup_1`").eval(&r));
}
