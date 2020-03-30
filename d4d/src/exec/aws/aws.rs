use crate::exec::aws::capabilities::Capabilities;
use crate::exec::{ExecCtx, Runner, Software};

use env::Env;
use std::path::Path;
use utils::result::Result;

#[derive(Debug)]
pub struct Aws<'c, 'e> {
    software: Software<'c>,
    env: &'e Env,
}

impl<'c, 'e> Aws<'c, 'e> {
    pub fn new(env: &'e Env, exec_ctx: &'c ExecCtx) -> Result<Self> {
        Ok(Self {
            software: Software::new("aws", exec_ctx)?,
            // TODO: provide region from global configuration
            env,
        })
    }

    pub fn fake(env: &'e Env, exec_ctx: &'c ExecCtx) -> Self {
        Self {
            software: Software::fake("aws", exec_ctx),
            env,
        }
    }

    fn cli_set_region(&mut self) -> Result<()> {
        let (_, region) = self.env.get("AWS_REGION")?;
        self.software.args(&["--region", region.as_str()]);
        Ok(())
    }

    pub fn cli_version(mut self) -> Runner<'c> {
        self.software.arg("--version");
        self.software.runner()
    }

    pub fn cli_cloudformation_package<IT, B, OT>(
        mut self,
        template_file: IT,
        deploy_bucket_name: B,
        template_pkg_file: OT,
    ) -> Result<Runner<'c>>
    where
        IT: AsRef<Path>,
        B: AsRef<str>,
        OT: AsRef<Path>,
    {
        self.cli_set_region()?;
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
        Ok(self.software.runner())
    }

    pub fn cli_cloudformation_deploy<T, S>(
        mut self,
        template_file: T,
        stack_name: S,
        capabilities: Capabilities,
    ) -> Result<Runner<'c>>
    where
        T: AsRef<Path>,
        S: AsRef<str>,
    {
        self.cli_set_region()?;
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
        Ok(self.software.runner())
    }

    pub fn cli_s3_bucket_exists<B: AsRef<str>>(mut self, bucket_name: B) -> Runner<'c> {
        self.software.args(&["s3api", "head-bucket"]);
        self.software.args(&["--bucket", bucket_name.as_ref()]);
        self.software.runner()
    }

    pub fn cli_s3_create_bucket<B: AsRef<str>>(mut self, bucket_name: B) -> Result<Runner<'c>> {
        self.cli_set_region()?;
        self.software.args(&[
            "s3",
            "mb",
            format!("s3://{}", bucket_name.as_ref()).as_str(),
        ]);
        Ok(self.software.runner())
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::aws::Aws;

    use crate::exec::aws::capabilities::{Capabilities, Capability};
    use crate::exec::{ExecCtx, Software};
    
    use env::Env;

    fn env() -> Env {
        let mut env = Env::new();
        env.add("AWS_REGION", "test-region");
        env
    }

    fn new_fake_aws<'ec>(env: &'ec Env, exec_ctx: &'ec ExecCtx) -> Aws<'ec, 'ec> {
        Aws {
            software: Software::fake("aws", exec_ctx),
            env,
        }
    }

    #[test]
    fn version() {
        let exec_ctx = ExecCtx::new();
        let env = self::env();
        let aws = new_fake_aws(&env, &exec_ctx);
        let runner = aws.cli_version();
        let args = runner.args();
        assert_eq!(args, &vec!["--version"])
    }

    #[test]
    fn cloudformation_package() {
        let exec_ctx = ExecCtx::new();
        let env = self::env();
        let aws = new_fake_aws(&env, &exec_ctx);
        let runner = aws
            .cli_cloudformation_package(
                "./template_name_file.yaml",
                "deploy_bucket_1",
                "template_name_file",
            )
            .unwrap();
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
        let env = self::env();
        let aws = new_fake_aws(&env, &exec_ctx);
        let runner = aws
            .cli_cloudformation_deploy(
                "./template_name_file.yaml",
                "stack_name",
                Capabilities::new(),
            )
            .unwrap();

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
        let env = self::env();
        let aws = new_fake_aws(&env, &exec_ctx);
        let mut capabilities = Capabilities::new();
        capabilities.add(Capability::CAPABILITY_IAM);
        capabilities.add(Capability::CAPABILITY_NAMED_IAM);
        let runner = aws
            .cli_cloudformation_deploy("./template_name_file.yaml", "stack_name", capabilities)
            .unwrap();

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
        let env = self::env();
        let aws = new_fake_aws(&env, &exec_ctx);
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
        let env = self::env();
        let aws = new_fake_aws(&env, &exec_ctx);
        let runner = aws.cli_s3_create_bucket("test-bucket").unwrap();
        let args = runner.args();
        assert_eq!(
            args,
            &vec!["--region", "test-region", "s3", "mb", "s3://test-bucket",]
        )
    }
}
