use crate::exec::output::Output;
use utils::result::Result;

#[derive(Debug)]
pub struct AwsOutputS3Exists {
    is_exists: bool,
}

impl AwsOutputS3Exists {
    pub fn is_exists(&self) -> bool {
        self.is_exists
    }
}

impl From<Output> for Result<AwsOutputS3Exists> {
    fn from(output: Output) -> Self {
        let mut is_exists = true;
        if let Some(err) = output.fail {
            if err.exit_code_eq(255)? {
                is_exists = false;
            }
        }
        Ok(AwsOutputS3Exists { is_exists })
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::aws_output::AwsOutputS3Exists;
    use crate::exec::output::Output;

    use utils::error::Error;
    use utils::result::Result;

    #[test]
    fn aws_out_put_s3exists() {
        let output = Output {
            fail: None,
            stderr: vec![],
            stdout: vec![],
        };
        let aws_out_put_s3exists: Result<AwsOutputS3Exists> = output.into();
        assert!(aws_out_put_s3exists.unwrap().is_exists());

        let output = Output {
            fail: Some(Error::from(Some(255))),
            stderr: vec![],
            stdout: vec![],
        };
        let aws_out_put_s3exists: Result<AwsOutputS3Exists> = output.into();
        assert!(!aws_out_put_s3exists.unwrap().is_exists());
    }
}
