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

    let local_project_file = &config.tmp_project_dir.join("d4d.yaml");
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
        region: us-east-3
"#;
    config
        .add_asset_project("./d4d.yaml", PROJECT_FILE_CONTENT)
        .unwrap();
    let _output = config.command().arg("init").output().unwrap();
    let local_project_file = &config.tmp_project_dir.join("d4d.yaml");
    let content = read_to_string(local_project_file).unwrap();
    assert_eq!(PROJECT_FILE_CONTENT, content.as_str());
}

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
    public_env_directory: "."
    provider:
      name: aws
      region: us-east-1
      template_path: "./template.yaml""#,
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
    provider:
        name: aws
        region: us-east-3
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
    provider:
        name: aws
        region: us-east-3
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

    // Project : p1
    config
        .add_asset_project(
            "./d4d.yaml",
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
            ".d4d/projects.yaml",
            format!(
                r#"---
projects: []"#
            ),
        )
        .unwrap();

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

#[test]
fn run_deploy() {
    let config = before("env", Assets::None).cli(CRATE_NAME);

    // Project : p1
    config
        .add_asset_project(
            "./d4d.yaml",
            r#"---
projects:
  - name: p1
    public_env_directory: "."
    provider:
        name: aws
        region: us-east-3
        template_path: "./template.yaml"
"#,
        )
        .unwrap();
    config
        .add_asset_project("./.dev", r#"AWS_S3_BUCKET_DEPLOY=bucket_1"#)
        .unwrap();

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
        .arg("deploy")
        .args(&["-p", "p1"])
        .args(&["-e", "dev"])
        .arg("--dry-run")
        .output()
        .unwrap();

    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!(
            r#"--dry-run
aws --region eu-west-3 cloudformation package --template-file {p}/./template.yaml --s3-bucket bucket_1 --output-template-file {p}/template.pkg.yaml
aws --region eu-west-3 cloudformation deploy --template-file {p}/template.pkg.yaml --stack-name p1-dev

"#,
            p = config.tmp_project_dir.to_string_lossy().trim()
        )
    )
}
