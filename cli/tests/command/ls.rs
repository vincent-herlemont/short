use short_utils::asset::Assets;
use short_utils::test::before;

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn ls_empty() {
    let config = before("add", Assets::None).cli(CRATE_NAME);

    // Project : empty
    config
        .add_asset_project(
            "./short.yaml",
            r#"---
projects: []"#,
        )
        .unwrap();
    config.add_asset_project("./template.yaml", r#""#).unwrap();

    let mut command = config.command();
    command.arg("ls");
    let output = command.output().unwrap();

    assert_eq!("\n", String::from_utf8(output.stdout).unwrap());
}

#[test]
fn ls() {
    let config = before("add", Assets::None).cli(CRATE_NAME);

    // Project : empty
    config
        .add_asset_project(
            "./short.yaml",
            r#"projects:
  - name: p1
    public_env_directory: sub_project
    provider:
        name: aws
        region: us-east-3
        template_path: "sub_project/template.yaml"
  - name: p2
    public_env_directory: sub_project
    provider:
        name: aws
        region: us-east-3
        template_path: "sub_project/template.yaml""#,
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
        .add_asset_home(
            ".short/projects.yaml",
            format!(
                r#"---
current_project:
  name: p1
  env: dev
projects: []"#
            ),
        )
        .unwrap();
    config.add_asset_project("./template.yaml", r#""#).unwrap();

    let mut command = config.command();
    command.arg("ls");
    let output = command.output().unwrap();

    assert_eq!(
        r#"> dev p1 sub_project/template.yaml
         dev
      p2 sub_project/template.yaml
         dev

"#,
        String::from_utf8(output.stdout).unwrap()
    );
}
