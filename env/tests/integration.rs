use env::Env;
use std::io::Cursor;

#[test]
fn empty() {
    let mut content = Cursor::new(br#""#);
    let env = Env::from(&mut content).unwrap();
    assert_eq!(format!("{}", env), "")
}

#[test]
fn once_var() {
    let mut content = Cursor::new(br#"A=a"#);
    let env = Env::from(&mut content).unwrap();
    assert_eq!(format!("{}", env), "A=a\n")
}

#[test]
fn name_end_with_space() {
    let mut content = Cursor::new(br#"A=a "#);
    let env = Env::from(&mut content).unwrap();
    assert_eq!(format!("{}", env), "A=a\n")
}

#[test]
fn name_start_with_space() {
    let mut content = Cursor::new(br#"A= a"#);
    let env = Env::from(&mut content).unwrap();
    assert_eq!(format!("{}", env), "A=a\n")
}

#[test]
fn value_end_with_space() {
    let mut content = Cursor::new(br#"A =a"#);
    let env = Env::from(&mut content).unwrap();
    assert_eq!(format!("{}", env), "A=a\n");
}

#[test]
fn value_start_with_space() {
    let mut content = Cursor::new(br#" A=a"#);
    let env = Env::from(&mut content).unwrap();
    assert_eq!(format!("{}", env), "A=a\n");
}

#[test]
fn value_with_space_inside() {
    let mut content = Cursor::new(br#"A B=a"#);
    let env = Env::from(&mut content);
    assert!(env.is_err());
}

#[test]
fn empty_comment() {
    let mut content = Cursor::new(br#"#"#);
    let env = Env::from(&mut content).unwrap();
    assert_eq!(format!("{}", env), "#\n")
}

#[test]
fn comment() {
    let mut content = Cursor::new(br#"#test"#);
    let env = Env::from(&mut content).unwrap();
    assert_eq!(format!("{}", env), "#test\n")
}

#[test]
fn multi_var() {
    let mut content = Cursor::new(
        br#"A=a
    B=b"#,
    );
    let env = Env::from(&mut content).unwrap();
    assert_eq!(format!("{}", env), "A=a\nB=b\n")
}

#[test]
fn multi_var_and_comment() {
    let mut content = Cursor::new(
        br#"A=a
#test
B=b"#,
    );
    let env = Env::from(&mut content).unwrap();
    assert_eq!(format!("{}", env), "A=a\n#test\nB=b\n")
}

#[test]
fn empty_lines() {
    let mut content = Cursor::new(
        br#"

"#,
    );
    let env = Env::from(&mut content).unwrap();
    assert_eq!(format!("{}", env), "\n\n")
}
