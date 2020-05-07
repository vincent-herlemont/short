use assert_cmd::output::{OutputError, OutputResult};
use std::process::Output;

pub mod environment;

#[macro_export]
macro_rules! println_output {
    ($v:ident) => {
        println!(
            "{}",
            String::from_utf8($v.stderr.clone()).expect("fail to read stderr")
        );
        println!("---------------------------");
        println!(
            "{}",
            String::from_utf8($v.stdout.clone()).expect("fail to read stdout")
        );
        println!("---------------------------");
        println!("{}", $v.status);
    };
}

#[macro_export]
macro_rules! println_result_output {
    ($v:ident) => {
        match $v {
            Ok(output) => {
                println_output!(output);
            }
            Err(outputError) => {
                println!("output error !!");
                println!("{}", outputError.to_string());
            }
        }
    };
}
