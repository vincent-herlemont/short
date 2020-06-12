mod test_utils;
use crate::test_utils::PROJECT_CFG_FILE;
use test_utils::init;

#[test]
fn generate_template() {
    let mut e = init("generate_template");
    e.add_file(
        PROJECT_CFG_FILE,
        r#"
setups: []
        "#,
    );
    e.setup();
    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("generate")
        .arg("setup_1")
        .arg("example")
        .args(&["-t", "test"])
        .assert()
        .to_string();

    println!("{}", &r);

    dbg!(e.tree());
}
