use cli_integration_test::IntegrationTestEnvironment;
use predicates::prelude::Predicate;
use predicates::str::contains;



mod test_utils;

#[test]
fn cmd_ls_settings() {
    let e = IntegrationTestEnvironment::new("cmd_ls_settings");

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .assert()
        .to_string();

    assert!(contains("fail to load cfg").eval(&r));
}

#[test]
fn cmd_ls() {
    let mut e = IntegrationTestEnvironment::new("cmd_ls_settings");

    e.add_file("template.yaml", "");
    e.add_file("setup_2/.example_1", "VAR1=VALUE1");
    e.add_file("setup_2/.example_2", "VAR1=VALUE1");
    e.add_file(
        "short.yml",
        r"#---
setups:
  - name: setup_1
    file: test.sh
    array_vars: {}
  - name: setup_2
    file: test.sh
    array_vars: {}
    public_env_dir: 'setup_2/'
    #",
    );
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .assert()
        .to_string();

    assert!(contains("setup_1").count(1).eval(&r));

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .args(&["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains("> setup_1").count(1).eval(&r));

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .args(&["-s", "setup_2"])
        .args(&["-e", "example_2"])
        .assert()
        .to_string();

    assert!(contains(">    example_2").count(1).eval(&r));
}
