use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    include_walk::from("./src/assets").to("./src/assets.rs")
}
