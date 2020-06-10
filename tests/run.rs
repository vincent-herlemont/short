use predicates::prelude::Predicate;
use predicates::str::contains;
use utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment};

mod utils;

#[test]
fn cmd_run() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_run");

    {
        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(
            &itew
                .get_rel_path(PathTestEnvironment::LocalEnvExample)
                .unwrap(),
            r#"VAR1=VALUE1"#,
        );
        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups:
  - name: setup_1
    file: run.sh
    array_vars:
      ALL: .*
    vars: [ VAR1 ]
        "#,
        );
        let run_file = itew.get_rel_path(PathTestEnvironment::Run).unwrap();
        e.add_file(
            &run_file,
            r#"#!/bin/bash
echo "TEST VAR1=$VAR1"
declare -p ALL
        "#,
        );
        e.setup();
        e.set_exec_permission(&run_file).unwrap();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let command = command
        .env("RUST_LOG", "debug")
        .arg("run")
        .args(&vec!["-s", "setup_1"])
        .args(&vec!["-e", "example"]);
    let r = command.assert().success().to_string();
    assert!(contains("#> TEST VAR1=VALUE1").count(1).eval(&r));
    assert!(contains("#> declare -x ALL=\" [VAR1]='VALUE1' \"")
        .count(1)
        .eval(&r));
}
