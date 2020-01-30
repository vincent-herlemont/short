mod resource;
mod d4d_error;
mod cloudformation;
mod path;

fn main() {
    let res = resource::get();
    println!("hello world ! {:?}",res);
}