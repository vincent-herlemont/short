use crate::utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment};

use predicates::prelude::Predicate;
use predicates::str::contains;

mod utils;

#[test]
fn cmd_new() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_init");
    {
        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(
            &itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap(),
            r#"
setups: []
        "#,
        );
        e.setup();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("new")
        .arg("setup_1")
        .assert()
        .success()
        .to_string();

    assert!(contains("new setup").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow_mut();
        let r = e.read_file(itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap());
        assert!(contains("setup_1").eval(&r));

        let r = e.read_file("project/run.sh");
        assert!(contains("declare -A all && eval all=($ALL)").eval(&r));
    }
}
