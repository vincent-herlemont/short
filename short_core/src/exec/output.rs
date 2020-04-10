use short_utils::error::Error;
use std::process::Output as StdOutPut;

#[derive(Debug)]
pub struct Output<C> {
    pub ctx: C,
    pub fail: Option<Error>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl<C> Output<C> {
    pub fn new(ctx: C, output: StdOutPut) -> Self {
        let mut exit_status_error = None;
        if !output.status.success() {
            exit_status_error = Some(Error::from(output.status.code()));
        }
        Self {
            ctx,
            stdout: output.stdout,
            stderr: output.stderr,
            fail: exit_status_error,
        }
    }
}
