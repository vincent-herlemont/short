use predicates::prelude::Predicate;


use predicates::prelude::predicate::path::exists;
use short::BIN_NAME;
use test_utils::init;
use test_utils::{HOME_DIR};

mod test_utils;

#[test]
fn check_update() {
    let e = init("cmd_env_dir");
    e.setup();

    let mut command = e.command(BIN_NAME).unwrap();
    let _r = command.assert().to_string();
    exists().eval(&e.path().unwrap().join(HOME_DIR).join(".last_update.json"));
}
