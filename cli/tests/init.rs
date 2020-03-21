use insta::assert_debug_snapshot;
use std::fs::read_to_string;
use utils::asset::Assets;
use utils::test::{before, ConfigPath};
const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn init() {
    let config = before("init", Assets::None).cli(CRATE_NAME);
    let output = config.command().arg("init").output().unwrap();
    assert_eq!("\n", String::from_utf8(output.stdout).unwrap());

    let content_dir = config.tree();
    assert_debug_snapshot!(content_dir);
}

#[test]
fn add() {
    let config = before("add", Assets::None).cli(CRATE_NAME);
    let mut command = config.command();
    command.arg("add").arg("my_project").arg("./template.yaml");
    let output = command.output().unwrap();
    assert_eq!(
        "project name : my_project \npath to template : ./template.yaml\n\n",
        String::from_utf8(output.stdout).unwrap()
    );

    let local_project_file = &config.tmp_project_dir.join("d4d.yaml");
    let content = read_to_string(local_project_file).unwrap();
    assert_eq!(
        r#"---
projects:
  - name: my_project
    template_path: "./template.yaml"
    public_env_directory: ".""#,
        content.as_str()
    );

    let global_project_file = &config.tmp_home_dir.join(".d4d/projects.yaml");
    let content = read_to_string(global_project_file).unwrap();
    assert_eq!(
        format!(
            r#"---
projects:
  - name: my_project
    path: {}"#,
            config.tmp_project_dir.to_string_lossy()
        ),
        content.as_str()
    );
    let content_dir = config.tree();
    assert_debug_snapshot!(content_dir);
}

#[test]
fn check_env_local() {
    let config = before("env", Assets::None).cli(CRATE_NAME);

    // Project : p1
    config
        .add_asset_project(
            "./d4d.yaml",
            r#"---
projects:
  - name: p1
    public_env_directory: "."
"#,
        )
        .unwrap();
    config.add_asset_project("./.dev", r#"VAR1=val1"#).unwrap();

    config
        .add_asset_home(
            ".d4d/projects.yaml",
            format!(
                r#"---
projects:
  - name: p1
    path: {}"#,
                config.tmp_project_dir.to_string_lossy()
            ),
        )
        .unwrap();

    let mut command = config.command();

    let output = command
        .arg("env")
        .arg("-c")
        .arg("-p")
        .arg("p1")
        .arg("-e")
        .arg("dev")
        .output()
        .unwrap();

    assert_eq!(
        r#"- p1
VAR1=val1

"#,
        String::from_utf8(output.stdout).unwrap()
    )
}

#[test]
fn check_env_private() {
    let config = before("env", Assets::None).cli(CRATE_NAME);

    // Project : p1
    config
        .add_asset_project(
            "./d4d.yaml",
            r#"---
projects:
  - name: p1
"#,
        )
        .unwrap();
    config
        .add_asset_private_env("./.dev", r#"VAR1=val1"#)
        .unwrap();

    config
        .add_asset_home(
            ".d4d/projects.yaml",
            format!(
                r#"---
projects:
  - name: p1
    path: {}
    private_env_directory: {}"#,
                config.tmp_project_dir.to_string_lossy(),
                config.tmp_private_env_dir.to_string_lossy()
            ),
        )
        .unwrap();

    let tree = config.tree();
    dbg!(tree);
    let mut command = config.command();

    let output = command
        .arg("env")
        .arg("-c")
        .arg("-p")
        .arg("p1")
        .arg("-e")
        .arg("dev")
        .output()
        .unwrap();

    assert_eq!(
        r#"- p1
VAR1=val1

"#,
        String::from_utf8(output.stdout).unwrap()
    )
}

#[test]
fn run_use() {
    let config = before("env", Assets::None).cli(CRATE_NAME);
    let mut command = config.command();
    let _output = command.arg("use").arg("p1").arg("dev").output().unwrap();
    let global_project_file = &config.tmp_home_dir.join(".d4d/projects.yaml");
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
