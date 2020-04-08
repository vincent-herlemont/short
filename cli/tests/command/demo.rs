



use utils::asset::Assets;
use utils::test::{before};

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn demo() {
    // DEMO indicatif
    let config = before("add", Assets::None).cli(CRATE_NAME);
    let mut command = config.command();
    command.arg("demo");
    //let output = command.output().unwrap();
}
