pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const BIN_NAME: &'static str = "short";

fn main() {
    println!("{}", BIN_NAME);
    println!("v{}", VERSION);
}
