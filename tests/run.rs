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
    array_vars: {}
        "#,
        );
        let run_file = itew.get_rel_path(PathTestEnvironment::Run).unwrap();
        e.add_file(
            &run_file,
            r#"#!/bin/bash
echo "TEST"
        "#,
        );
        e.setup();
        e.set_exec_permission(&run_file).unwrap();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("run")
        .args(&vec!["-s", "setup_1"])
        .args(&vec!["-e", "example"])
        .assert()
        .to_string();

    println!("{}", r);
}
