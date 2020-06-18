use crate::test_utils::{HOME_CFG_FILE, PROJECT_CFG_FILE, PROJECT_DIR};
use predicates::prelude::Predicate;
use predicates::str::contains;
use test_utils::init;

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
    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command
        .env("RUST_LOG", "debug")
        .arg("generate")
        .arg("-l")
        .assert()
        .to_string();

    assert!(
        contains("aws-sam   https://github.com/vincent-herlemont/aws-sam-short-template.git")
            .eval(&r)
    );
}
