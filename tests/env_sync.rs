use predicates::prelude::Predicate;
use predicates::prelude::*;
use predicates::str::contains;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment};

mod utils;

#[test]
fn cmd_env_sync() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_use");
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
        e.setup();
        thread::sleep(Duration::from_secs(2));
        e.add_file(
            &source_local_env_example2,
            r#"VAR1=VALUE1
VAR2=VALUE2
"#,
        );
        e.setup();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("sync")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    contains("files synchronized").eval(&r);

    {
        let e = itew.e();
        let e = e.borrow();
        let target = e.read_file(&target_local_env_example);
        let source = e.read_file(&source_local_env_example2);
        assert_eq!(target, source);
    }
}