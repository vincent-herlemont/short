use crate::exec::aws::capabilities::Capabilities;
use crate::exec::{ExecCtx, Runner, Software};
use crate::project::provider::AwsCfg;
use std::path::Path;
use utils::result::Result;

#[derive(Debug)]
pub struct Aws<'s, 'a> {
    software: Software<'s>,
    aws_cfg: &'a AwsCfg,
}

impl<'s, 'a> Aws<'s, 'a> {
    pub fn new(aws_cfg: &'a AwsCfg, exec_ctx: &'s ExecCtx) -> Result<Self> {
        Ok(Self {
            software: Software::new("aws", exec_ctx)?,
            // TODO: provide region from global configuration
            aws_cfg,
        })
    }

    pub fn fake(aws_cfg: &'a AwsCfg, exec_ctx: &'s ExecCtx) -> Self {
        Self {
            software: Software::fake("aws", exec_ctx),
            aws_cfg,
        }
    }

    fn cli_set_region(&mut self) {
        self.software.args(&["--region", self.aws_cfg.region()])
    }

    pub fn cli_version(mut self) -> Runner<'s> {
        self.software.arg("--version");
        self.software.runner()
    }

    pub fn cli_cloudformation_package<IT, B, OT>(
        mut self,
        template_file: IT,
        deploy_bucket_name: B,
        template_pkg_file: OT,
    ) -> Runner<'s>
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
    ) -> Runner<'s>
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

    pub fn cli_s3_bucket_exists<B: AsRef<str>>(mut self, bucket_name: B) -> Runner<'s> {
        self.software.args(&["s3api", "head-bucket"]);
        self.software.args(&["--bucket", bucket_name.as_ref()]);
        self.software.runner()
    }

    pub fn cli_s3_create_bucket<B: AsRef<str>>(mut self, bucket_name: B) -> Runner<'s> {
        self.cli_set_region();
        self.software.args(&[
            "s3api",
            "create-bucket",
            "--bucket",
            bucket_name.as_ref(),
            // "--create-bucket-configuration",
            // format!("LocationConstraint={}", self.aws_cfg.region()).as_ref(),
        ]);
        self.software.runner()
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::aws::Aws;

    use crate::exec::aws::capabilities::{Capabilities, Capability};
    use crate::exec::{ExecCtx, Software};
    use crate::project::provider::AwsCfg;

    fn new_fake_aws<'a>(aws_cfg: &'a AwsCfg, exec_ctx: &'a ExecCtx) -> Aws<'a, 'a> {
        Aws {
            software: Software::fake("aws", exec_ctx),
            aws_cfg,
        }
    }

    #[test]
    fn version() {
        let exec_ctx = ExecCtx::new();
        let aws_cfg = AwsCfg::new("test-region");
        let aws = new_fake_aws(&aws_cfg, &exec_ctx);
        let runner = aws.cli_version();
        let args = runner.args();
        assert_eq!(args, &vec!["--version"])
    }

    #[test]
    fn cloudformation_package() {
        let exec_ctx = ExecCtx::new();
        let aws_cfg = AwsCfg::new("test-region");
        let aws = new_fake_aws(&aws_cfg, &exec_ctx);
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
        let exec_ctx = ExecCtx::new();
        let aws_cfg = AwsCfg::new("test-region");
        let aws = new_fake_aws(&aws_cfg, &exec_ctx);
        let runner = aws.cli_cloudformation_deploy(
            "./template_name_file.yaml",
            "stack_name",
            Capabilities::new(),
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

        let exec_ctx = ExecCtx::new();
        let aws_cfg = AwsCfg::new("test-region");
        let aws = new_fake_aws(&aws_cfg, &exec_ctx);
        let mut capabilities = Capabilities::new();
        capabilities.add(Capability::CAPABILITY_IAM);
        capabilities.add(Capability::CAPABILITY_NAMED_IAM);
        let runner =
            aws.cli_cloudformation_deploy("./template_name_file.yaml", "stack_name", capabilities);

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

    #[test]
    fn s3_bucket_exists() {
        let exec_ctx = ExecCtx::new();
        let aws_cfg = AwsCfg::new("test-region");
        let aws = new_fake_aws(&aws_cfg, &exec_ctx);
        let runner = aws.cli_s3_bucket_exists("test-bucket");
        let args = runner.args();
        assert_eq!(
            args,
            &vec!["s3api", "head-bucket", "--bucket", "test-bucket"]
        )
    }

    #[test]
    fn create_bucket() {
        let exec_ctx = ExecCtx::new();
        let aws_cfg = AwsCfg::new("test-region");
        let aws = new_fake_aws(&aws_cfg, &exec_ctx);
        let runner = aws.cli_s3_create_bucket("test-bucket");
        let args = runner.args();
        assert_eq!(
            args,
            &vec![
                "--region",
                "test-region",
                "s3api",
                "create-bucket",
                "--bucket",
                "test-bucket",
            ]
        )
    }
}
