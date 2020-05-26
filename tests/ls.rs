use cli_integration_test::IntegrationTestEnvironment;
use predicates::prelude::Predicate;
use predicates::str;

#[test]
fn cmd_ls_settings() {
    let e = IntegrationTestEnvironment::new("cmd_help");
    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .assert()
        .to_string();
    let isSetup = str::is_match(r"setup None\n").unwrap();
    assert_eq!(isSetup.eval(r.as_str()), true);
    let isEnv = str::is_match(r"env None\n").unwrap();
    assert_eq!(isEnv.eval(r.as_str()), true);
}

#[test]
fn cmd_ls() {
    let mut e = IntegrationTestEnvironment::new("cmd_help");
    e.add_file("template.yaml", "");
    e.add_file("setup_2/.example_1", "VAR1=VALUE1");
    e.add_file("setup_2/.example_2", "VAR1=VALUE1");
    e.add_file(
        "short.yml",
        r"#---
setups:
  - name: setup_1
    file: test.sh
    env_groups: {}
  - name: setup_2
    file: test.sh
    env_groups: {}
    public_env_dir: 'setup_2/'
    #",
    );
    e.setup();
    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .args(vec!["-s", "setup_2", "-e", "example_1"])
        .assert()
        .to_string();

    println!("{}", r);
}
