use test_utils::HOME_CFG_FILE;
use test_utils::init;

mod test_utils;

#[test]
fn cmd_init() {
    let e = init("cmd_init");
    e.setup();

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let _r = command
        .env("RUST_LOG", "debug")
        .arg("init")
        .assert()
        .success()
        .to_string();

    // Check the new local cfg file
    assert!(e.file_exists(HOME_CFG_FILE));

    let mut command = e.command(env!("CARGO_PKG_NAME"));
    let r = command.env("RUST_LOG", "debug").arg("init").assert();
    r.failure();
}
