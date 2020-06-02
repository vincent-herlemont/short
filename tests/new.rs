use crate::utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment};

use predicates::prelude::predicate::path::exists;
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
        .arg("example")
        .assert()
        .success()
        .to_string();

    assert!(contains("new setup").eval(&r));

    {
        let e = itew.e();
        let e = e.borrow_mut();
        let local_env_example = itew
            .get_rel_path(PathTestEnvironment::LocalEnvExample)
            .unwrap();
        exists().eval(&local_env_example);

        let r = e.read_file(itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap());
        assert!(contains("setup_1").eval(&r));

        let r = e.read_file("project/run.sh");
        assert!(contains("declare -A all && eval all=($ALL)").eval(&r));

        let r = e.read_file(itew.get_rel_path(PathTestEnvironment::GlobalCfg).unwrap());
        assert!(contains("current").count(1).eval(&r));
        assert!(contains("setup: setup_1").count(1).eval(&r));
        assert!(contains("env: example").count(1).eval(&r));
    }
}
