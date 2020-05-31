use predicates::prelude::Predicate;
use predicates::str::contains;
use utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment};

mod utils;

#[test]
fn cmd_show_no_setup_no_env() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_show");

    {
        let e = itew.e();
        let mut e = e.borrow_mut();

        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups: []
        "#,
        );
        e.setup();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("show")
        .assert()
        .to_string();

    assert!(contains("no setup is configured. You can use").eval(&r));
}

#[test]
fn cmd_show_no_setup() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_show");

    {
        let local_cfg_abs_path = itew.get_abs_path(PathTestEnvironment::LocalCfg).unwrap();
        let e = itew.e();
        let mut e = e.borrow_mut();

        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups: []
        "#,
        );

        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::GlobalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            format!(
                r#"
projects:
  - file: {}
    current:
        setup: setup_1
    setups: []
        "#,
                local_cfg_abs_path.to_string_lossy()
            ),
        );
        e.setup();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("show")
        .assert()
        .to_string();

    assert!(contains("no env is configured for \"setup_1\"").eval(&r));
}

#[test]
fn cmd_show() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_show");

    {
        let local_cfg_abs_path = itew.get_abs_path(PathTestEnvironment::LocalCfg).unwrap();
        let e = itew.e();
        let mut e = e.borrow_mut();

        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups: []
        "#,
        );

        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::GlobalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            format!(
                r#"
projects:
  - file: {}
    current:
        setup: setup_1
        env: example
    setups: []
        "#,
                local_cfg_abs_path.to_string_lossy()
            ),
        );
        e.setup();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("show")
        .assert()
        .to_string();

    assert!(contains("your current setup is \"setup_1\":\"example\"").eval(&r));
}
