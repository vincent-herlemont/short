use predicates::prelude::Predicate;
use predicates::str::contains;
use test_utils::init;
use test_utils::{
    HOME_CFG_FILE, PRIVATE_ENV_DEV_FILE, PRIVATE_ENV_DIR, PROJECT_CFG_FILE, PROJECT_ENV_DIR,
    PROJECT_ENV_EXAMPLE_1_FILE, PROJECT_ENV_EXAMPLE_2_FILE, PROJECT_RUN_FILE,
};

mod test_utils;

#[test]
fn cmd_show_no_setup_no_env() {
    let mut e = init("cmd_show_no_setup_no_env");

    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: []
        "#,
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("show")
        .assert()
        .to_string();

    assert!(contains("no setup is configured. You can use").eval(&r));
}

#[test]
fn cmd_show_no_setup() {
    let mut e = init("cmd_show_no_setup");

    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: []
        "#,
    );

    e.add_file(
        HOME_CFG_FILE,
        format!(
            r#"
projects:
  - file: {file}
    current:
        setup: setup_1
    setups: []
        "#,
            file = e.path().join(PROJECT_CFG_FILE).to_string_lossy()
        ),
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("show")
        .assert()
        .to_string();

    assert!(contains("no env is configured for \"setup_1\"").eval(&r));
}

#[test]
fn cmd_show() {
    let mut e = init("cmd_show");

    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: []
        "#,
    );

    e.add_file(
        HOME_CFG_FILE,
        format!(
            r#"
projects:
  - file: {file}
    current:
        setup: setup_1
        env: example
    setups: []
        "#,
            file = e.path().join(PROJECT_CFG_FILE).to_string_lossy()
        ),
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("show")
        .assert()
        .to_string();

    assert!(contains("your current setup is \"setup_1\":\"example\"").eval(&r));
}
