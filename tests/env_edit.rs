
use predicates::prelude::Predicate;
use predicates::str::contains;
use std::path::{PathBuf};
use utils::{IntegrationTestEnvironmentWrapper, PathTestEnvironment};

mod utils;

#[test]
fn cmd_env_edit() {
    let itew = IntegrationTestEnvironmentWrapper::init_all("cmd_use");
    let local_env_example = itew
        .get_abs_path(PathTestEnvironment::LocalEnvExample)
        .unwrap();
    let run_rel_file = PathBuf::from("mock_editor.sh");
    let mut run_abs_file = PathBuf::new();
    {
        let e = itew.e();
        let mut e = e.borrow_mut();
        e.add_file(&local_env_example, r#"VAR1=VALUE1"#);
        let local_cfg_file = itew.get_rel_path(PathTestEnvironment::LocalCfg).unwrap();
        e.add_file(
            &local_cfg_file,
            r#"
setups:
  - name: setup_1
    file: run.sh
        "#,
        );

        e.add_file(
            &run_rel_file,
            r#"#!/bin/bash
echo -e "\nVAR2=VALUE2" >> $1
        "#,
        );
        run_abs_file = e.path().join(&run_rel_file);
        e.setup();
        e.set_exec_permission(run_rel_file).unwrap();
    }

    let mut command = itew.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("env")
        .arg("edit")
        .arg("example")
        .args(vec!["-s", "setup_1"])
        .args(vec![
            "-e",
            run_abs_file.to_string_lossy().into_owned().as_str(),
        ])
        .assert()
        .to_string();

    contains("`example` edited").count(1).eval(&r);

    {
        let e = itew.e();
        let e = e.borrow_mut();
        let r = e.read_file(local_env_example);
        contains("VAR2=VALUE2").count(1).eval(&r);
    }
}
