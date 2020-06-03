use crate::utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment};

use predicates::prelude::Predicate;
use predicates::str::contains;

mod utils;

#[test]
fn cmd_rename() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_init");
    {
        let local_cfg = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(
            &local_cfg,
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
        .arg("rename")
        .arg("setup_1")
        .arg("setup_2")
        .assert()
        .success()
        .to_string();

    assert!(contains("setup renamed").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow_mut();
        let r = e.read_file(itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap());
        assert!(!contains("setup_1").eval(&r));
        assert!(contains("setup_2").eval(&r));

        let r = e.read_file(itew.get_rel_path(PathTestEnvironment::GlobalCfg).unwrap());
        assert!(!contains("setup_1").eval(&r));
        assert!(contains("setup_2").count(1).eval(&r));
    }
}

#[test]
fn cmd_rename_with_use() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_init");
    {
        let local_cfg = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        let local_abs_cfg = itew.get_abs_path(PathTestEnvironment::LocalCfg).unwrap();
        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(
            &local_cfg,
            r#"
setups:
  - name: setup_1
    file: run.sh
    array_vars: {}
        "#,
        );
        e.add_file(
            &itew.get_rel_path(PathTestEnvironment::GlobalCfg).unwrap(),
            format!(
                r#"---
projects:
  - file: {}
    current:
      setup: setup_1
    setups:
      - name: setup_1
            "#,
                &local_abs_cfg.to_string_lossy()
            ),
        );
        e.setup();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("rename")
        .arg("setup_1")
        .arg("setup_2")
        .assert()
        .success()
        .to_string();

    assert!(contains("setup renamed").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow_mut();
        let r = e.read_file(itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap());
        assert!(!contains("setup_1").eval(&r));
        assert!(contains("setup_2").eval(&r));

        let r = e.read_file(itew.get_rel_path(PathTestEnvironment::GlobalCfg).unwrap());
        assert!(!contains("setup_1").eval(&r));
        assert!(contains("setup_2").count(2).eval(&r));
    }
}
