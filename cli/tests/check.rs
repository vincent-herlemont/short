use utils::asset::Assets;
use utils::test::before;

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

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
