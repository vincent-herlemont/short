use crate::cli::terminal::emoji;

pub fn message(msg: &str) {
    println!("{}", msg);
}

pub fn success(msg: &str) {
    println!("{} {}", emoji::CHECK, msg)
}

pub fn info(msg: &str) {
    println!("{} {}", emoji::RIGHT_POINTER, msg)
}

pub fn good_info(msg: &str) {
    println!("{} {}", emoji::PERSON_TIPPING_HANG, msg)
}

pub fn bad_info(msg: &str) {
    println!("{} {}", emoji::PERSON_POUTING, msg)
}
