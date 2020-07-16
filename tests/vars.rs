use predicates::prelude::Predicate;
use predicates::str::contains;

use short::BIN_NAME;
use test_utils::init;
use test_utils::{
    HOME_CFG_FILE, PROJECT_CFG_FILE, PROJECT_ENV_EXAMPLE_1_FILE, PROJECT_ENV_EXAMPLE_2_FILE,
};

mod test_utils;

#[test]
fn cmd_vars_multiple_envs() {
    let mut e = init("cmd_var");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  setup_1:
    file: run.sh
    array_vars:
        ALL: "VAR.*"
    vars: [ VAR_A, CONFIG ]
    "#,
    );
    e.add_file(
        PROJECT_ENV_EXAMPLE_1_FILE,
        r#"VAR_A=VALUE1
VAR_B=VALUE1
CONFIG=AZE
"#,
    );
    e.add_file(
        PROJECT_ENV_EXAMPLE_2_FILE,
        r#"VAR_A=VALUE2
VAR_B=VALUE2
CONFIG=AZE
"#,
    );
    e.setup();
    e.set_update_file_time(PROJECT_ENV_EXAMPLE_1_FILE).unwrap();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("vars")
        .args(&["-s", "setup_1"])
        .assert()
        .to_string();

    assert!(contains(r#"you can set a current env with the command"#,).eval(&r));

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("vars")
        .args(&["-s", "setup_1"])
        .args(&["-e", "example1"])
        .assert()
        .to_string();

    assert!(contains(
        r#"| example1 
 all         | ALL (VAR.*)            
             | VAR_A       | VALUE1 
             | VAR_B       | VALUE1 
 var_a       | VAR_A       | VALUE1 
 config      | CONFIG      | AZE 
 short_setup | SHORT_SETUP | setup_1 
 short_env   | SHORT_ENV   | example1"#,
    )
    .eval(&r));

    e.add_file(
        HOME_CFG_FILE,
        format!(
            r#"
projects:
  - file: {file}
    current:
        setup: setup_1
        env: example2
    setups: {{}}
        "#,
            file = e.path().unwrap().join(PROJECT_CFG_FILE).to_string_lossy()
        ),
    );
    e.setup();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("vars")
        .args(&["-s", "setup_1"])
        .args(&["-e", "example1"])
        .assert()
        .to_string();

    assert!(contains(
        r#"| example1 | example2 
 all         | ALL (VAR.*)                       
             | VAR_A       | VALUE1   | VALUE2 
             | VAR_B       | VALUE1   | VALUE2 
 var_a       | VAR_A       | VALUE1   | VALUE2 
 config      | CONFIG      | AZE      | AZE 
 short_setup | SHORT_SETUP | setup_1  | setup_1 
 short_env   | SHORT_ENV   | example1 | example2"#,
    )
    .eval(&r));
}

#[test]
fn cmd_vars_case() {
    let mut e = init("cmd_var");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups:
  setup_1:
    file: run.sh
    array_vars:
      ALL: 
        pattern: "VAR.*"
        case: CamelCase
    vars: [ VAR_A, CONFIG ]
    "#,
    );
    e.add_file(
        PROJECT_ENV_EXAMPLE_1_FILE,
        r#"VAR_A=VALUE1
VAR_B=VALUE1
CONFIG=AZE
"#,
    );
    e.add_file(
        PROJECT_ENV_EXAMPLE_2_FILE,
        r#"VAR_A=VALUE2
VAR_B=VALUE2
CONFIG=AZE
"#,
    );
    e.setup();
    e.set_update_file_time(PROJECT_ENV_EXAMPLE_1_FILE).unwrap();

    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("vars")
        .args(&["-s", "setup_1"])
        .args(&["-e", "example1"])
        .assert()
        .to_string();

    assert!(contains(
        r#"| example1   
 all         | ALL (VAR.*) `CamelCase`  
             | VarA        | VALUE1 
             | VarB        | VALUE1 
 var_a       | VAR_A       | VALUE1 
 config      | CONFIG      | AZE 
 short_setup | SHORT_SETUP | setup_1 
 short_env   | SHORT_ENV   | example1"#,
    )
    .eval(&r));
}
