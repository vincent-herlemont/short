use insta::assert_debug_snapshot;
use std::fs::read_to_string;
use utils::asset::Assets;
use utils::test::{before, ConfigPath};

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn add() {
    let config = before("add", Assets::None).cli(CRATE_NAME);

    // Project : empty
    config
        .add_asset_project(
            "./d4d.yaml",
            r#"---
projects: []"#,
        )
        .unwrap();
    config.add_asset_project("./template.yaml", r#""#).unwrap();

    let mut command = config.command();
    command.arg("add").arg("my_project").arg("./template.yaml");
    let output = command.output().unwrap();

    assert_eq!(
        "project name : my_project \npath to template : template.yaml\n\n",
        String::from_utf8(output.stdout).unwrap()
    );

    const PROJECT_FILE_CONTENT: &'static str = r#"---
projects:
  - name: my_project
    public_env_directory: ""
    provider:
      name: aws
      template_path: template.yaml"#;
    let local_project_file = &config.tmp_project_dir.join("d4d.yaml");
    let content = read_to_string(local_project_file).unwrap();
    assert_eq!(PROJECT_FILE_CONTENT, content.as_str());

    let global_project_file_content: String = format!(
        r#"---
projects:
  - name: my_project
    path: {}"#,
        config.tmp_project_dir.to_string_lossy()
    );
    let global_project_file = &config.tmp_home_dir.join(".d4d/projects.yaml");
    let content = read_to_string(global_project_file).unwrap();
    assert_eq!(global_project_file_content, content.as_str());

    let content_dir = config.tree();
    assert_debug_snapshot!(content_dir);

    // Make sure to not insert the same project twice.
    let mut command = config.command();
    command.arg("add").arg("my_project").arg("./template.yaml");
    let output = command.output().unwrap();

    assert_eq!(
        "project my_project already exists\n",
        String::from_utf8(output.stderr).unwrap()
    );

    let local_project_file = &config.tmp_project_dir.join("d4d.yaml");
    let content = read_to_string(local_project_file).unwrap();
    assert_eq!(PROJECT_FILE_CONTENT, content.as_str());

    let global_project_file = &config.tmp_home_dir.join(".d4d/projects.yaml");
    let content = read_to_string(global_project_file).unwrap();
    assert_eq!(global_project_file_content, content.as_str());
}

#[test]
fn add_in_sub_directory() {
    let config = before("add_in_sub_directory", Assets::None).cli(CRATE_NAME);
    // Project : empty
    config
        .add_asset_project(
            "./d4d.yaml",
            r#"---
projects: []"#,
        )
        .unwrap();
    config
        .add_asset_project("./sub_directory/template.yaml", r#""#)
        .unwrap();

    let mut command = config.command();
    command
        .current_dir(config.tmp_project_dir.join("sub_directory"))
        .arg("add")
        .arg("my_project")
        .arg("./template.yaml");

    let output = command.output().unwrap();
    assert_eq!(
        "project name : my_project \npath to template : sub_directory/template.yaml\n\n",
        String::from_utf8(output.stdout).unwrap()
    );

    const PROJECT_FILE_CONTENT: &'static str = r#"---
projects:
  - name: my_project
    public_env_directory: sub_directory
    provider:
      name: aws
      template_path: sub_directory/template.yaml"#;
    let local_project_file = &config.tmp_project_dir.join("d4d.yaml");
    let content = read_to_string(local_project_file).unwrap();
    assert_eq!(PROJECT_FILE_CONTENT, content.as_str());

    let global_project_file_content: String = format!(
        r#"---
projects:
  - name: my_project
    path: {}"#,
        config.tmp_project_dir.to_string_lossy()
    );
    let global_project_file = &config.tmp_home_dir.join(".d4d/projects.yaml");
    let content = read_to_string(global_project_file).unwrap();
    assert_eq!(global_project_file_content, content.as_str());
}
