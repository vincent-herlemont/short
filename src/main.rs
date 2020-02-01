mod cloudformation;
mod lib;
mod resource;

fn main() {
    let res = resource::get();
    println!("hello world ! {:?}", res);
}
