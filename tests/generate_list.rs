use predicates::prelude::Predicate;
use predicates::str::contains;

use test_utils::init;

use crate::test_utils::PROJECT_CFG_FILE;
use short::BIN_NAME;

mod test_utils;

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
    let mut command = e.command(BIN_NAME).unwrap();
    let r = command
        .env("RUST_LOG", "debug")
        .arg("generate")
        .arg("-l")
        .assert()
        .to_string();

    assert!(contains(
        "aws-node-sam  https://github.com/vincent-herlemont/aws-node-sam-short-template"
    )
    .eval(&r));
}
