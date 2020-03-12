use insta::assert_debug_snapshot;
use std::fs::read_to_string;
use utils::asset::Assets;
use utils::test::before;

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn init() {
    let config = before("init", Assets::None).cli(CRATE_NAME);
    let output = config.command().arg("init").output().unwrap();
    assert_eq!("\n", String::from_utf8(output.stdout).unwrap());

    // TODO : make helper
    let mut content_dir: Vec<_> = walkdir::WalkDir::new(&config.tmp_dir)
        .into_iter()
        .map(|e| {
            let e = e.unwrap();
            e.into_path()
                .strip_prefix(&config.tmp_dir)
                .unwrap()
                .to_path_buf()
        })
        .collect();
    content_dir.sort();
    assert_debug_snapshot!(content_dir);
}

#[test]
fn add() {
    let config = before("add", Assets::None).cli(CRATE_NAME);
    let mut command = config.command();
    command
        .arg("add")
        .arg("my_project")
        .arg("./path/to/template.yaml");
    let output = command.output().unwrap();
    assert_eq!(
        "project name : my_project \npath to template : ./path/to/template.yaml\n\n",
        String::from_utf8(output.stdout).unwrap()
    );

    let local_project_file = &config.tmp_project_dir.join("d4d.yaml");
    let content = read_to_string(local_project_file).unwrap();
    assert_eq!(
        r#"---
projects:
  - name: my_project
    template_path: "./path/to/template.yaml"
    public_env_directory: "./path/to""#,
        content.as_str()
    )
}
