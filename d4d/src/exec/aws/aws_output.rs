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

use serde::Deserialize;
use utils::error::Error;

#[derive(Debug, Deserialize)]
pub struct AwsOutputS3BucketLocation {
    #[serde(rename = "LocationConstraint")]
    location_constraint: String,
}

impl From<Output> for Result<AwsOutputS3BucketLocation> {
    fn from(output: Output) -> Self {
        let aws_output_s3bucket_location =
            serde_yaml::from_slice(output.stdout.as_slice()).map_err(|e| Error::from(e))?;
        Ok(aws_output_s3bucket_location)
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::aws_output::{AwsOutputS3BucketLocation, AwsOutputS3Exists};
    use crate::exec::output::Output;
    use utils::error::Error;
    use utils::result::Result;

    #[test]
    fn aws_output_s3bucket_location() {
        let output = Output {
            fail: None,
            stderr: vec![],
            stdout: br#"{
    "LocationConstraint": "us-west-1"
}"#
            .to_vec(),
        };

        let aws_output_s3bucket_location: Result<AwsOutputS3BucketLocation> = output.into();
        let aws_output_s3bucket_location = aws_output_s3bucket_location.unwrap();
        dbg!(&aws_output_s3bucket_location);
    }

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
