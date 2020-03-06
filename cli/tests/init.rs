use utils::asset::Assets;
use utils::test::before;

#[test]
fn init() {
    let config = before("init", Assets::None).cli();
    let output = config.command().arg("init").output().unwrap();
    assert_eq!("[]\n".as_bytes(), output.stdout.as_slice());
}
