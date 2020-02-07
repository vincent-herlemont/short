#[macro_use]
mod lib;
mod cloudformation;

fn main() {
    let res = lib::test::get_resource();
    println!("hello world ! {:?}", res);
}
