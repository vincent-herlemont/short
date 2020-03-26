use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    include_walk::from("./src/assets").to("./src/assets.rs")
}
