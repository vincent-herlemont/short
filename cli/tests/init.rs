use std::collections::HashMap;
use std::env::current_exe;
use std::process::Command;
use utils::asset::Assets;
use utils::test::before;

const HOME: &'static str = "home/.keep";
const PROJECT: &'static str = "project/.keep";

#[test]
fn mock_home_directory() {
    let mut assets = HashMap::new();
    assets.insert(HOME, "");
    assets.insert(PROJECT, "");
    let config = before("slurp", Assets::All(assets));
    let home_path = config.tmp_dir.join(HOME).parent().unwrap().to_path_buf();
    let project_path = config.tmp_dir.join(PROJECT).parent().unwrap().to_path_buf();

    let current_exec = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
        .join("cli");

    let output = Command::new(current_exec)
        .arg("init")
        .current_dir(project_path)
        .env("HOME", home_path)
        .output()
        .expect("failed to execute process");

    assert_eq!("[]\n".as_bytes(), output.stdout.as_slice());
}
