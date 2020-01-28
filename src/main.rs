mod resource;
mod d4d_error;

fn main() {
    let res = resource::get();
    println!("hello world ! {:?}",res);
}

pub mod cloudformation {

}