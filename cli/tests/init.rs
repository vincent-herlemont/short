use utils::asset::Assets;
use utils::test::before;

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn init() {
    let config = before("init", Assets::None).cli(CRATE_NAME);
    let output = config.command().arg("init").output().unwrap();
    assert_eq!("\n", String::from_utf8(output.stdout).unwrap());
}
