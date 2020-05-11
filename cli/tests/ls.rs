#[macro_use]
extern crate short_utils;

use predicates::prelude::Predicate;
use predicates::str;

use short_utils::integration_test::environment::IntegrationTestEnvironment;

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
    e.add_file("setup_2/template.yaml", "");
    e.add_file(
        "short.yml",
        r"#---
setups:
  - name: 'setup_1'
    provider:
      name: cloudformation
      template: ./template_1.yaml
  - name: setup_2'
    public_env_directory: 'setup_2/'
    provider:
      name: cloudformation
      template: setup_2/template.yaml
    #",
    );
    e.setup();
    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("ls")
        .assert()
        .to_string();
    println!("{}", r);
}
