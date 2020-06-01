use predicates::prelude::predicate::path::exists;
use predicates::prelude::Predicate;
use predicates::str::contains;
use std::path::PathBuf;
use utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment, ENVDIR, PROJECT};

mod utils;

#[test]
fn cmd_env_new_public() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_use");

    {
        let local_cfg_abs_path = itew.get_abs_path(PathTestEnvironment::LocalCfg).unwrap();
        let global_env_dev_file = itew
            .get_abs_path(PathTestEnvironment::GlobalEnvDev)
            .unwrap();
        let global_env_dir = global_env_dev_file.parent().unwrap();

        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(&global_env_dev_file, r#"VAR1=VALUE1"#);
        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups:
  - name: setup_1
    file: run.sh
    public_env_dir: env
        "#,
        );
        e.setup();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("new")
        .arg("example")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("env `example` created").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow();
        assert!(exists().eval(&e.path().join(PathBuf::from(PROJECT).join("env/.example"))));
    }
}

#[test]
fn cmd_env_new_private() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_use");

    {
        let local_cfg_abs_path = itew.get_abs_path(PathTestEnvironment::LocalCfg).unwrap();
        let global_env_dev_file = itew
            .get_abs_path(PathTestEnvironment::GlobalEnvDev)
            .unwrap();
        let global_env_dir = global_env_dev_file.parent().unwrap();

        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(&global_env_dev_file, r#"VAR1=VALUE1"#);
        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups:
  - name: setup_1
    file: run.sh
        "#,
        );

        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::GlobalCfg).unwrap();

        e.add_file(
            &local_cfg_file,
            format!(
                r#"
        projects:
          - file: {file}
            current:
                setup: setup_1
            setups:
              - name: setup_1
                private_env_dir: {private_env_dir}
                "#,
                file = local_cfg_abs_path.to_string_lossy(),
                private_env_dir = global_env_dir.to_string_lossy()
            ),
        );
        e.setup();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("new")
        .arg("dev")
        .args(vec!["-p", "-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("env `dev` created").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow();
        assert!(exists().eval(&e.path().join(PathBuf::from(ENVDIR).join(".dev"))));
    }
}
