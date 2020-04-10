use crate::exec::output::Output;
use short_utils::result::Result;

#[derive(Debug)]
pub struct AwsOutputS3Exists {
    is_exists: bool,
}

impl AwsOutputS3Exists {
    pub fn is_exists(&self) -> bool {
        self.is_exists
    }
}

impl<C> From<Output<C>> for Result<AwsOutputS3Exists> {
    fn from(output: Output<C>) -> Self {
        let mut is_exists = true;
        if let Some(err) = output.fail {
            if err.exit_code_eq(255)? {
                is_exists = false;
            }
        }
        Ok(AwsOutputS3Exists { is_exists })
    }
}

use crate::exec::aws::aws_command::AwsCtxS3BucketLocation;

use serde::Deserialize;
use short_utils::error::Error;

#[derive(Debug, Deserialize)]
pub struct AwsOutputS3BucketLocation {
    #[serde(rename = "LocationConstraint")]
    location_constraint: String,
}

impl From<Output<AwsCtxS3BucketLocation>> for Result<AwsOutputS3BucketLocation> {
    fn from(output: Output<AwsCtxS3BucketLocation>) -> Self {
        let aws_output_s3bucket_location: AwsOutputS3BucketLocation =
            serde_yaml::from_slice(output.stdout.as_slice()).map_err(|e| Error::from(e))?;
        let ctx = output.ctx;
        if ctx.region != aws_output_s3bucket_location.location_constraint {
            return Err(Error::from(format!(
                "the bucket region {} differ from deployment region {}",
                aws_output_s3bucket_location.location_constraint, ctx.region
            )));
        }
        Ok(aws_output_s3bucket_location)
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::aws_command::AwsCtxS3BucketLocation;
    use crate::exec::aws::aws_output::{AwsOutputS3BucketLocation, AwsOutputS3Exists};
    use crate::exec::output::Output;
    use crate::exec::EmptyCtx;
    use short_utils::error::Error;
    use short_utils::result::Result;

    #[test]
    fn aws_output_s3bucket_location() {
        let output = Output {
            ctx: AwsCtxS3BucketLocation {
                region: String::from("us-west-1"),
            },
            fail: None,
            stderr: vec![],
            stdout: br#"{
    "LocationConstraint": "us-west-1"
}"#
            .to_vec(),
        };

        let aws_output_s3bucket_location: Result<AwsOutputS3BucketLocation> = output.into();
        let aws_output_s3bucket_location = aws_output_s3bucket_location.unwrap();
        assert_eq!(
            aws_output_s3bucket_location.location_constraint,
            String::from("us-west-1")
        );

        let output = Output {
            ctx: AwsCtxS3BucketLocation {
                region: String::from("us-west-1"),
            },
            fail: None,
            stderr: vec![],
            stdout: br#"{
    "LocationConstraint": "us-west-2"
}"#
            .to_vec(),
        };

        let aws_output_s3bucket_location: Result<AwsOutputS3BucketLocation> = output.into();
        let aws_output_s3bucket_location = aws_output_s3bucket_location;
        assert!(aws_output_s3bucket_location.is_err());
    }

    #[test]
    fn aws_out_put_s3exists() {
        let output = Output {
            ctx: EmptyCtx {},
            fail: None,
            stderr: vec![],
            stdout: vec![],
        };
        let aws_out_put_s3exists: Result<AwsOutputS3Exists> = output.into();
        assert!(aws_out_put_s3exists.unwrap().is_exists());

        let output = Output {
            ctx: EmptyCtx {},
            fail: Some(Error::from(Some(255))),
            stderr: vec![],
            stdout: vec![],
        };
        let aws_out_put_s3exists: Result<AwsOutputS3Exists> = output.into();
        assert!(!aws_out_put_s3exists.unwrap().is_exists());
    }
}
