use predicates::prelude::Predicate;
use predicates::str::contains;
use utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment};

mod utils;

#[test]
fn cmd_use() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_use");

    {
        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(
            &itew
                .get_rel_path(PathTestEnvironment::LocalEnvExample)
                .unwrap(),
            r#"VAR1=VALUE1"#,
        );
        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups:
  - name: setup_1
    file: run.sh
    array_vars: {}
        "#,
        );
        e.setup();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("use")
        .arg("setup_1")
        .assert()
        .to_string();

    assert!(contains("your current setup is `setup_1`").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow();
        let content = e.read_file(itew.get_rel_path(PathTestEnvironment::GlobalCfg).unwrap());
        assert!(contains("setup: setup_1").count(1).eval(&content));
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("use")
        .arg("setup_1")
        .arg("example")
        .assert()
        .to_string();

    assert!(contains("your current setup is `setup_1:example`").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow();
        let content = e.read_file(itew.get_rel_path(PathTestEnvironment::GlobalCfg).unwrap());
        assert!(contains("setup: setup_1").count(1).eval(&content));
        assert!(contains("env: example").count(1).eval(&content));
    }
}
