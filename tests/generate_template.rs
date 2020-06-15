use crate::test_utils::{HOME_CFG_FILE, PROJECT_CFG_FILE, PROJECT_DIR};
use predicates::prelude::Predicate;
use predicates::str::contains;
use test_utils::init;

mod test_utils;

#[test]
fn generate_template() {
    let mut e = init("generate_template");
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
        .arg("test_setup_1")
        .args(&["-t", "test"])
        .assert()
        .to_string();

    assert!(contains("generate setup `test_setup_1`:`dev`").eval(&r));

    let env_dev = e.path().join(PROJECT_DIR).join("env/.dev");
    assert!(env_dev.exists());
    let run_sh = e.path().join(PROJECT_DIR).join("run.sh");
    assert!(run_sh.exists());

    let r = e.read_file(PROJECT_CFG_FILE);
    assert_eq!(
        r#"---
setups:
  - name: test_setup_1
    public_env_dir: "./env/"
    file: run.sh
    array_vars:
      all: ".*"
    vars:
      - SETUP_NAME"#,
        &r
    );

    let r = e.read_file(HOME_CFG_FILE);
    assert!(contains("setup: test_setup_1").count(1).eval(&r));
    assert!(contains("env: dev").count(1).eval(&r));
    assert!(contains("test_setup_1").count(2).eval(&r));
}
