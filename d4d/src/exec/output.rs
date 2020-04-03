use std::process::Output as StdOutPut;
use utils::error::Error;

#[derive(Debug)]
pub struct Output {
    pub fail: Option<Error>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl Output {
    pub fn new(output: StdOutPut) -> Self {
        let mut exist_status_error = None;
        if !output.status.success() {
            exist_status_error = Some(Error::from(output.status.code()));
        }
        Self {
            stdout: output.stdout,
            stderr: output.stderr,
            fail: exist_status_error,
        }
    }
}
