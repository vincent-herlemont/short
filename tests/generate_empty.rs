use crate::test_utils::{
    HOME_CFG_FILE, PROJECT_CFG_FILE, PROJECT_ENV_EXAMPLE_1_FILE, PROJECT_RUN_FILE,
};

use predicates::prelude::Predicate;
use predicates::str::contains;
use test_utils::init;

mod test_utils;

#[test]
fn cmd_generate() {
    let mut e = init("cmd_generate");

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
        .arg("generate")
        .arg("setup_1")
        .arg("example1")
        .assert()
        .success()
        .to_string();

    assert!(contains("generate setup").eval(&r));
    debug_assert!(e.file_exists(PROJECT_ENV_EXAMPLE_1_FILE));

    let r = e.read_file(PROJECT_CFG_FILE);
    assert!(contains("setup_1").eval(&r));

    let r = e.read_file(PROJECT_RUN_FILE);
    assert!(contains("declare -A all && eval all=($ALL)").eval(&r));

    let r = e.read_file(HOME_CFG_FILE);
    assert!(contains("current").count(1).eval(&r));
    assert!(contains("setup: setup_1").count(1).eval(&r));
    assert!(contains("env: example").count(1).eval(&r));
}

#[test]
fn cmd_generate_with_existing_env() {
    let mut e = init("cmd_generate_with_existing_env");
    let local_env_example_content = r#"VAR1=VALUE1"#;
    e.add_file(PROJECT_ENV_EXAMPLE_1_FILE, local_env_example_content);
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
        .arg("generate")
        .arg("setup_1")
        .arg("example")
        .assert()
        .success()
        .to_string();

    assert!(contains("generate setup").eval(&r));

    assert!(e.file_exists(PROJECT_ENV_EXAMPLE_1_FILE));
    let r = e.read_file(PROJECT_ENV_EXAMPLE_1_FILE);
    assert_eq!(r, local_env_example_content);

    let r = e.read_file(PROJECT_CFG_FILE);
    assert!(contains("setup_1").eval(&r));

    let r = e.read_file(PROJECT_RUN_FILE);
    assert!(contains("declare -A all && eval all=($ALL)").eval(&r));

    let r = e.read_file(HOME_CFG_FILE);
    assert!(contains("current").count(1).eval(&r));
    assert!(contains("setup: setup_1").count(1).eval(&r));
    assert!(contains("env: example").count(1).eval(&r));
}