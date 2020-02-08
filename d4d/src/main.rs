use utils::test::get_assets;
mod assets;
mod cloudformation;

fn main() {
    let res = get_assets();
    println!("hello world ! {:?}", res);
}
