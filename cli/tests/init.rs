use short_utils::asset::Assets;
use short_utils::test::before;

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn deploy_duplicate_project() {
    let config = before("env", Assets::None).cli(CRATE_NAME);

    // Project : p1
    config
        .add_asset_project(
            "./short.yaml",
            r#"---
projects:
  - name: p1
    public_env_directory: sub_project
    provider:
        name: aws
        region: us-east-3
        template_path: "sub_project/template.yaml"
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
        String::from_utf8(output.stderr).unwrap(),
        String::from("some project(s) are duplicate on configuration file\n")
    );
}
