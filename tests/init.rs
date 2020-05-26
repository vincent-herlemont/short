use crate::utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment};
use predicates::prelude::predicate::path::exists;
use predicates::prelude::Predicate;

mod utils;

#[test]
fn cmd_init() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_init");

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let _r = command
        .env("RUST_LOG", "debug")
        .arg("init")
        .assert()
        .success()
        .to_string();

    // Check the new local cfg file
    let path = &itew.get_abs_path(PathTestEnvironment::LocalCfg).unwrap();
    assert!(exists().eval(path));

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command.env("RUST_LOG", "debug").arg("init").assert();
    r.failure();
}
