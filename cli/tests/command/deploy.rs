use std::fs::read_to_string;
use utils::asset::Assets;
use utils::test::before;

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn run_aws() {
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
        template_path: "./template.yaml"
"#,
        )
        .unwrap();
    config
        .add_asset_project(
            "./.dev",
            r#"AWS_S3_BUCKET_DEPLOY=bucket_1
AWS_REGION=us-east-3"#,
        )
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
        .arg("run")
        .args(&["-p", "p1"])
        .args(&["-e", "dev"])
        .arg("--dry-run")
        .output()
        .unwrap();

    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!(
            r#"aws s3api head-bucket --bucket bucket_1
aws --region us-east-3 s3api get-bucket-location --bucket bucket_1
aws --region us-east-3 cloudformation package --template-file {p}/./template.yaml --s3-bucket bucket_1 --output-template-file {p}/template.pkg.yaml
aws --region us-east-3 cloudformation deploy --template-file {p}/template.pkg.yaml --stack-name p1-dev

"#,
            p = config.tmp_project_dir.to_string_lossy().trim()
        )
    )
}

#[test]
fn run_aws_sync() {
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
        .add_asset_project("./.dev", r#"AWS_S3_BUCKET_run=bucket_1"#)
        .unwrap();

    let mut command = config.command();
    let _output = command
        .arg("run")
        .args(&["-p", "p1"])
        .args(&["-e", "dev"])
        .arg("--dry-run")
        .output()
        .unwrap();

    let global_project_file_content: String = format!(
        r#"---
projects:
  - name: p1
    path: {}"#,
        config.tmp_project_dir.to_string_lossy()
    );
    let global_project_file = &config.tmp_home_dir.join(".d4d/projects.yaml");
    let content = read_to_string(global_project_file).unwrap();
    assert_eq!(global_project_file_content, content.as_str());
}

#[test]
fn run_aws_sync_sub_command() {
    let config = before("env", Assets::None).cli(CRATE_NAME);

    // Project : p1
    config
        .add_asset_project(
            "./d4d.yaml",
            r#"---
projects:
  - name: p1
    public_env_directory: sub_project
    provider:
        name: aws
        region: us-east-3
        template_path: "sub_project/template.yaml"
"#,
        )
        .unwrap();
    config
        .add_asset_project(
            "sub_project/.dev",
            r#"AWS_S3_BUCKET_DEPLOY=bucket_1
AWS_REGION=us-east-3"#,
        )
        .unwrap();
    config
        .add_asset_project("sub_project/template.yaml", r#""#)
        .unwrap();

    let mut command = config.command();
    let output = command
        .current_dir(config.tmp_project_dir.join("sub_project"))
        .arg("run")
        .args(&["-p", "p1"])
        .args(&["-e", "dev"])
        .arg("--dry-run")
        .output()
        .unwrap();

    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!(
            r#"aws s3api head-bucket --bucket bucket_1
aws --region us-east-3 s3api get-bucket-location --bucket bucket_1
aws --region us-east-3 cloudformation package --template-file {p}/sub_project/template.yaml --s3-bucket bucket_1 --output-template-file {p}/sub_project/template.pkg.yaml
aws --region us-east-3 cloudformation deploy --template-file {p}/sub_project/template.pkg.yaml --stack-name p1-dev

"#,
            p = config.tmp_project_dir.to_string_lossy().trim()
        )
    )
}
