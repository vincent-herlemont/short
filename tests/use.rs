use predicates::prelude::Predicate;
use predicates::str::contains;
use test_utils::init;
use test_utils::{
    HOME_CFG_FILE, PRIVATE_ENV_DEV_FILE, PRIVATE_ENV_DIR, PROJECT_CFG_FILE, PROJECT_ENV_DIR,
    PROJECT_ENV_EXAMPLE_1_FILE, PROJECT_ENV_EXAMPLE_2_FILE, PROJECT_RUN_FILE,
};

mod test_utils;

#[test]
fn cmd_use() {
    let mut e = init("cmd_use");

    e.add_file(PROJECT_ENV_EXAMPLE_1_FILE, r#"VAR1=VALUE1"#);
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  - name: setup_1
    file: run.sh
    array_vars: {}
        "#,
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("use")
        .arg("setup_1")
        .assert()
        .to_string();

    assert!(contains("your current setup is `setup_1`").eval(&r));

    let content = e.read_file(HOME_CFG_FILE);
    assert!(contains("setup: setup_1").count(1).eval(&content));

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("use")
        .arg("setup_1")
        .arg("example1")
        .assert()
        .to_string();

    assert!(contains("your current setup is `setup_1:example1`").eval(&r));

    let content = e.read_file(HOME_CFG_FILE);
    assert!(contains("setup: setup_1").count(1).eval(&content));
    assert!(contains("env: example1").count(1).eval(&content));
}

#[test]
fn cmd_use_with_private() {
    let mut e = init("cmd_use_with_private");
    e.add_file(PROJECT_ENV_EXAMPLE_1_FILE, r#"VAR1=VALUE1"#);
    e.add_file(PRIVATE_ENV_DEV_FILE, r#"VAR1=VALUE1"#);
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  - name: setup_1
    file: run.sh
    array_vars: {}
        "#,
    );

    e.add_file(
        HOME_CFG_FILE,
        format!(
            r#"
projects:
  - file: {file}
    setups:
      - name: setup_1
        private_env_dir: {private_env_dir}
                "#,
            file = e.path().join(PROJECT_CFG_FILE).to_string_lossy(),
            private_env_dir = e.path().join(PRIVATE_ENV_DIR).to_string_lossy()
        ),
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("use")
        .arg("setup_1")
        .arg("example1")
        .assert()
        .to_string();

    assert!(contains("your current setup is `setup_1:example1`").eval(&r));

    let content = e.read_file(HOME_CFG_FILE);
    assert!(contains("setup: setup_1").count(1).eval(&content));
    assert!(contains("env: example1").count(1).eval(&content));

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("use")
        .arg("setup_1")
        .arg("dev")
        .assert()
        .to_string();

    assert!(contains("your current setup is `setup_1:dev`").eval(&r));

    let content = e.read_file(HOME_CFG_FILE);
    assert!(contains("setup: setup_1").count(1).eval(&content));
    assert!(contains("env: dev").count(1).eval(&content));
}
