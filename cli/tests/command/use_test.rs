use std::fs::read_to_string;
use utils::asset::Assets;
use utils::test::before;

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn run_use() {
    let config = before("env", Assets::None).cli(CRATE_NAME);

    // Project : p1
    config
        .add_asset_project(
            "./short.yaml",
            r#"---
projects: []
"#,
        )
        .unwrap();
    config
        .add_asset_private_env("./.dev", r#"VAR1=val1"#)
        .unwrap();

    config
        .add_asset_home(
            ".short/projects.yaml",
            format!(
                r#"---
projects: []"#
            ),
        )
        .unwrap();

    let mut command = config.command();
    let _output = command.arg("use").arg("p1").arg("dev").output().unwrap();

    let global_project_file = &config.tmp_home_dir.join(".short/projects.yaml");
    let content = read_to_string(global_project_file).unwrap();
    assert_eq!(
        r#"---
current_project:
  name: p1
  env: dev
projects: []"#,
        content
    );
}
