#[macro_use]
mod lib;
mod cloudformation;
mod resource;

fn main() {
    let res = resource::get();
    println!("hello world ! {:?}", res);
}
