use crate::cli::terminal::emoji;

pub fn message(msg: &str) {
    println!("{}", msg);
}

pub fn success(msg: &str) {
    println!("{} {}", emoji::CHECK, msg)
}
