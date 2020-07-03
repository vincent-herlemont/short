use std::path::PathBuf;

use predicates::prelude::Predicate;
use predicates::str::contains;

use test_utils::init;
use test_utils::{
    HOME_CFG_FILE, PRIVATE_ENV_DEV_FILE, PRIVATE_ENV_DIR, PROJECT_CFG_FILE, PROJECT_ENV_DIR,
};

mod test_utils;

#[test]
fn cmd_new_public() {
    let mut e = init("cmd_env_new_public");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  - name: setup_1
    file: run.sh
    public_env_dir: env
        "#,
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("new")
        .arg("example1")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("env `example1` created").eval(&r));

    e.file_exists(PathBuf::from(PROJECT_ENV_DIR).join(".example1"));
}

#[test]
fn cmd_new_private() {
    let mut e = init("cmd_env_new_private");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  - name: setup_1
    file: run.sh
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
    setups:
      - name: setup_1
        private_env_dir: {private_env_dir}
                "#,
            file = e.path().unwrap().join(PROJECT_CFG_FILE).to_string_lossy(),
            private_env_dir = e.path().unwrap().join(PRIVATE_ENV_DIR).to_string_lossy()
        ),
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("new")
        .arg("dev")
        .args(vec!["-p", "-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("env `dev` created").eval(&r));
    assert!(e.file_exists(PRIVATE_ENV_DEV_FILE));

    let r = e.read_file(HOME_CFG_FILE);
    assert!(contains("env: dev").count(1).eval(&r));
}

#[test]
fn cmd_new_public_with_sync() {
    let mut e = init("cmd_env_new_public_with_sync");
    let initial_env_file = PathBuf::from(PROJECT_ENV_DIR).join(".initial");
    let initial_env_content = r#"VAR1=VAR1
"#;
    e.add_file(&initial_env_file, initial_env_content);
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  - name: setup_1
    file: run.sh
    public_env_dir: env
        "#,
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("new")
        .arg("example")
        .arg("--copy")
        .args(vec!["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("env `example` created").eval(&r));

    let new_env_file = PathBuf::from(PROJECT_ENV_DIR).join(".example");
    assert!(e.file_exists(&new_env_file));
    let r = e.read_file(&new_env_file);
    assert_eq!(r, initial_env_content);
}

#[test]
fn cmd_new_duplicate_cross_public_private() {
    let mut e = init("cmd_env_new_public_with_sync");
    e.add_file(PRIVATE_ENV_DEV_FILE, "");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  - name: setup_1
    file: run.sh
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
    setups:
      - name: setup_1
        private_env_dir: {private_env_dir}
                "#,
            file = e.path().unwrap().join(PROJECT_CFG_FILE).to_string_lossy(),
            private_env_dir = e.path().unwrap().join(PRIVATE_ENV_DIR).to_string_lossy()
        ),
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME")).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("new")
        .arg("dev")
        .args(vec!["-s", "setup_1"])
        .assert()
        .failure()
        .to_string();

    assert!(contains("already exists").eval(&r));
}
