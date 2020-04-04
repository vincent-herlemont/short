use std::process::Output as StdOutPut;
use utils::error::Error;

#[derive(Debug)]
pub struct Output<C> {
    pub ctx: C,
    pub fail: Option<Error>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl<C> Output<C> {
    pub fn new(ctx: C, output: StdOutPut) -> Self {
        let mut exist_status_error = None;
        if !output.status.success() {
            exist_status_error = Some(Error::from(output.status.code()));
        }
        Self {
            ctx,
            stdout: output.stdout,
            stderr: output.stderr,
            fail: exist_status_error,
        }
    }
}
