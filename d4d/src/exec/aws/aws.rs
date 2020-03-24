use crate::exec::aws::capabilities::Capabilities;
use crate::exec::{Runner, Software};
use std::path::Path;
use utils::result::Result;

struct Aws {
    software: Software,
    region: String,
}

impl Aws {
    pub fn new() -> Result<Self> {
        Ok(Self {
            software: Software::new("aws")?,
            // TODO: provide region from global configuration
            region: String::from("eu-west-3"),
        })
    }

    fn cli_set_region(&mut self) {
        self.software.args(&["--region", self.region.as_str()])
    }

    pub fn cli_version(mut self) -> Runner {
        self.software.arg("--version");
        self.software.runner()
    }

    pub fn cli_cloudformation_package<IT, B, OT>(
        mut self,
        template_file: IT,
        deploy_bucket_name: B,
        template_pkg_file: OT,
    ) -> Runner
    where
        IT: AsRef<Path>,
        B: AsRef<str>,
        OT: AsRef<Path>,
    {
        self.cli_set_region();
        self.software.args(&[
            "cloudformation",
            "package",
            "--template-file",
            template_file.as_ref().to_string_lossy().trim(),
            "--s3-bucket",
            deploy_bucket_name.as_ref(),
            "--output-template-file",
            template_pkg_file.as_ref().to_string_lossy().trim(),
        ]);
        self.software.runner()
    }

    pub fn cli_cloudformation_deploy<T, S>(
        mut self,
        template_file: T,
        stack_name: S,
        capabilities: Capabilities,
    ) -> Runner
    where
        T: AsRef<Path>,
        S: AsRef<str>,
    {
        self.cli_set_region();
        self.software.args(&[
            "cloudformation",
            "deploy",
            "--template-file",
            template_file.as_ref().to_string_lossy().trim(),
            "--stack-name",
            stack_name.as_ref(),
        ]);
        if let Some(capabilities) = capabilities.to_strings() {
            self.software.arg("--capabilities");
            self.software.args(capabilities);
        }
        self.software.runner()
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::aws::Aws;
    use crate::exec::aws::capabilities::Capabilities;
    use crate::exec::aws::capabilities::Capability::{CAPABILITY_IAM, CAPABILITY_NAMED_IAM};
    use crate::exec::Software;

    fn new_fake_aws() -> Aws {
        Aws {
            software: Software::fake("aws"),
            region: String::from("test-region"),
        }
    }

    #[test]
    fn version() {
        let aws = new_fake_aws();
        let runner = aws.cli_version();
        let args = runner.args();
        assert_eq!(args, &vec!["--version"])
    }

    #[test]
    fn cloudformation_package() {
        let aws = new_fake_aws();
        let runner = aws.cli_cloudformation_package(
            "./template_name_file.yaml",
            "deploy_bucket_1",
            "template_name_file",
        );
        let args = runner.args();
        assert_eq!(
            args,
            &vec![
                "--region",
                "test-region",
                "cloudformation",
                "package",
                "--template-file",
                "./template_name_file.yaml",
                "--s3-bucket",
                "deploy_bucket_1",
                "--output-template-file",
                "template_name_file"
            ]
        )
    }

    #[test]
    fn cloudformation_deploy() {
        let aws = new_fake_aws();
        let runner = aws.cli_cloudformation_deploy(
            "./template_name_file.yaml",
            "stack_name",
            Capabilities::new(&[]),
        );

        let args = runner.args();
        assert_eq!(
            args,
            &vec![
                "--region",
                "test-region",
                "cloudformation",
                "deploy",
                "--template-file",
                "./template_name_file.yaml",
                "--stack-name",
                "stack_name",
            ]
        );

        let aws = new_fake_aws();
        let runner = aws.cli_cloudformation_deploy(
            "./template_name_file.yaml",
            "stack_name",
            Capabilities::new(&[CAPABILITY_IAM, CAPABILITY_NAMED_IAM]),
        );

        let args = runner.args();
        assert_eq!(
            args,
            &vec![
                "--region",
                "test-region",
                "cloudformation",
                "deploy",
                "--template-file",
                "./template_name_file.yaml",
                "--stack-name",
                "stack_name",
                "--capabilities",
                "CAPABILITY_IAM",
                "CAPABILITY_NAMED_IAM"
            ]
        );
    }
}
