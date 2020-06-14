use predicates::prelude::Predicate;

use predicates::str::contains;

use std::thread;
use std::time::Duration;
use utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment};

mod utils;

#[test]
fn cmd_env_sync_add_empty() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_env_sync_add_empty");
    let target_local_env_example = itew
        .get_abs_path(PathTestEnvironment::LocalEnvExample)
        .unwrap();
    let source_local_env_example2 = itew
        .get_abs_path(PathTestEnvironment::LocalEnvExample2)
        .unwrap();
    {
        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(
            &target_local_env_example,
            r#"VAR1=VALUE1
"#,
        );
        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups:
  - name: setup_1
    file: run.sh
        "#,
        );
        e.add_file(
            &source_local_env_example2,
            r#"VAR1=VALUE1
VAR2=VALUE2
"#,
        );
        e.setup();
        thread::sleep(Duration::from_secs(2));
        e.set_update_file_time(&source_local_env_example2).unwrap();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("sync")
        .arg("--empty")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("files synchronized").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow();
        let target = e.read_file(&target_local_env_example);
        let source = e.read_file(&source_local_env_example2);
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
}

#[test]
fn cmd_env_sync_add_copy() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_env_sync_add_copy");
    let target_local_env_example = itew
        .get_abs_path(PathTestEnvironment::LocalEnvExample)
        .unwrap();
    let source_local_env_example2 = itew
        .get_abs_path(PathTestEnvironment::LocalEnvExample2)
        .unwrap();
    {
        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(
            &target_local_env_example,
            r#"VAR1=VALUE1
"#,
        );
        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups:
  - name: setup_1
    file: run.sh
        "#,
        );
        e.add_file(
            &source_local_env_example2,
            r#"VAR1=VALUE1
VAR2=VALUE2
"#,
        );
        e.setup();
        thread::sleep(Duration::from_secs(2));
        e.set_update_file_time(&source_local_env_example2).unwrap();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("sync")
        .arg("--copy")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("files synchronized").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow();
        let target = e.read_file(&target_local_env_example);
        let source = e.read_file(&source_local_env_example2);
        assert_eq!(target, source);
    }
}

#[test]
fn cmd_env_sync_delete() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_env_sync_delete");
    let target_local_env_example = itew
        .get_abs_path(PathTestEnvironment::LocalEnvExample)
        .unwrap();
    let source_local_env_example2 = itew
        .get_abs_path(PathTestEnvironment::LocalEnvExample2)
        .unwrap();
    {
        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(
            &target_local_env_example,
            r#"VAR1=VALUE1
VAR3=VALUE3
"#,
        );
        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups:
  - name: setup_1
    file: run.sh
        "#,
        );
        e.add_file(
            &source_local_env_example2,
            r#"VAR1=VALUE1
"#,
        );
        e.setup();
        thread::sleep(Duration::from_secs(2));
        e.set_update_file_time(&source_local_env_example2).unwrap();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("sync")
        .arg("--delete")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("files synchronized").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow();
        let target = e.read_file(&target_local_env_example);
        let source = e.read_file(&source_local_env_example2);
        assert_eq!(target, source);
    }
}

#[test]
fn cmd_env_sync_no_delete() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_env_sync_no_delete");
    let target_local_env_example = itew
        .get_abs_path(PathTestEnvironment::LocalEnvExample)
        .unwrap();
    let source_local_env_example2 = itew
        .get_abs_path(PathTestEnvironment::LocalEnvExample2)
        .unwrap();
    let initial_target = r#"VAR1=VALUE1
VAR3=VALUE3
"#;
    let inital_source = r#"VAR1=VALUE1
"#;
    {
        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(&target_local_env_example, &initial_target);
        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups:
  - name: setup_1
    file: run.sh
        "#,
        );
        e.add_file(&source_local_env_example2, &inital_source);
        e.setup();
        thread::sleep(Duration::from_secs(2));
        e.set_update_file_time(&source_local_env_example2);
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("sync")
        .arg("--no_delete")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("you have not allowed to delete var `VAR3`:`VALUE3` in example").eval(&r));
    assert!(
        contains("env must be sync, please change it manually or run \"short env sync\"").eval(&r)
    );

    {
        let e = itew.e();
        let e = e.borrow();
        let target = e.read_file(&target_local_env_example);
        let source = e.read_file(&source_local_env_example2);
        assert_eq!(target, initial_target);
        assert_eq!(source, inital_source);
    }
}
