/// Hello lib
///
/// ```
/// use lib::hello_lib;
/// let hl = hello_lib();
/// dbg!(hl);
/// ```
pub fn hello_lib() -> String {
    "hello from lib".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
