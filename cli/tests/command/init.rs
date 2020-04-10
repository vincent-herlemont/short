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

    let local_project_file = &config.tmp_project_dir.join("short.yaml");
    let content = read_to_string(local_project_file).unwrap();
    assert_eq!(
        r#"---
projects: []"#,
        content.as_str()
    );

    // project file must not to be modify
    const PROJECT_FILE_CONTENT: &'static str = r#"---
projects:
  - name: p1
    public_env_directory: "."
    provider:
        name: aws
"#;
    config
        .add_asset_project("./short.yaml", PROJECT_FILE_CONTENT)
        .unwrap();
    let _output = config.command().arg("init").output().unwrap();
    let local_project_file = &config.tmp_project_dir.join("short.yaml");
    let content = read_to_string(local_project_file).unwrap();
    assert_eq!(PROJECT_FILE_CONTENT, content.as_str());
}
