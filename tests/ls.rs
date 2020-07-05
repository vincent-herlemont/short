use std::path::PathBuf;

use cli_integration_test::IntegrationTestEnvironment;
use predicates::prelude::Predicate;
use predicates::str::contains;

use short::cli::terminal::emoji::RIGHT_POINTER;
use test_utils::init;
use test_utils::{
    HOME_CFG_FILE, PRIVATE_ENV_DEV_FILE, PRIVATE_ENV_DIR, PROJECT_CFG_FILE, PROJECT_ENV_DIR,
};

mod test_utils;

#[test]
fn cmd_ls_settings() {
    let e = IntegrationTestEnvironment::new("cmd_ls_settings");

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .assert()
        .to_string();

    assert!(contains("fail to load cfg").eval(&r));
}

#[test]
fn cmd_ls() {
    let mut e = init("cmd_ls_settings");

    e.add_file("template.yaml", "");
    e.add_file(
        PathBuf::from(PROJECT_ENV_DIR).join(".example1"),
        "VAR1=VALUE1",
    );
    e.add_file(
        PathBuf::from(PROJECT_ENV_DIR).join(".example2"),
        "VAR1=VALUE1",
    );
    e.add_file(PathBuf::from(PRIVATE_ENV_DEV_FILE), "VAR1=VALUE1");
    e.add_file(
        PROJECT_CFG_FILE,
        r"#---
setups:
  setup_1:
    file: test.sh
    array_vars: {}
  setup_2:
    file: test.sh
    array_vars: {}
    public_env_dir: env/
#",
    );
    e.add_file(
        HOME_CFG_FILE,
        format!(
            r"
projects:
  - file: {file}
    setups:
      setup_1:
        private_env_dir: {private_env_dir}
    ",
            file = e.path().unwrap().join(PROJECT_CFG_FILE).to_string_lossy(),
            private_env_dir = e.path().unwrap().join(PRIVATE_ENV_DIR).to_string_lossy()
        ),
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .assert()
        .to_string();

    assert!(contains("setup_1 (test.sh)").count(1).eval(&r));
    assert!(contains(format!(
        "dev ({})",
        e.path()
            .unwrap()
            .join(PRIVATE_ENV_DEV_FILE)
            .to_string_lossy()
    ))
    .count(1)
    .eval(&r));
    assert!(contains("setup_2 (test.sh)").count(1).eval(&r));
    assert!(contains("example1 (env/.example1)").count(1).eval(&r));
    assert!(contains("example2 (env/.example2)").count(1).eval(&r));

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .args(&["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains(format!("{} setup_1", RIGHT_POINTER))
        .count(1)
        .eval(&r));

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .args(&["-s", "setup_2"])
        .args(&["-e", "example2"])
        .assert()
        .to_string();

    assert!(contains(format!("{}    example2", RIGHT_POINTER))
        .count(1)
        .eval(&r));
}
