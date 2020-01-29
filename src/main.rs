mod resource;
mod d4d_error;
mod cloudformation;

fn main() {
    let res = resource::get();
    println!("hello world ! {:?}",res);
}