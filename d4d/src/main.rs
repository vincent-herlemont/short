use lib::test::get_resource;
mod cloudformation;

fn main() {
    let res = get_resource();
    println!("hello world ! {:?}", res);
}
