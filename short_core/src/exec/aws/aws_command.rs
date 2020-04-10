use crate::exec::aws::capabilities::{Capabilities, Capability};
use crate::exec::aws::workflow::{
    AwsWorkflow, ENV_AWS_CAPABILITY_IAM, ENV_AWS_CAPABILITY_NAMED_IAM,
};
use crate::exec::{EmptyCtx, Runner, Software};

use crate::exec::aws::parameter_overrides::ParameterOverrides;
use utils::result::Result;

#[derive(Debug)]
pub struct CliAws<'p, 'e, 'c> {
    aws_workflow: AwsWorkflow<'p, 'e, 'c>,
    software: Software<'c>,
}

impl<'p, 'e, 'c> CliAws<'p, 'e, 'c> {
    pub fn new(aws_workflow: AwsWorkflow<'p, 'e, 'c>) -> Result<Self> {
        let exec_ctx = aws_workflow.exec_ctx();
        Ok(Self {
            software: Software::new("aws", exec_ctx)?,
            aws_workflow,
        })
    }

    fn set_region(&mut self) -> Result<()> {
        let region = self.aws_workflow.region()?;
        self.software.args(&["--region", region.as_str()]);
        Ok(())
    }

    pub fn version(mut self) -> Runner<'c, EmptyCtx> {
        self.software.arg("--version");
        self.software.runner(EmptyCtx {})
    }

    pub fn cloudformation_package(mut self) -> Result<Runner<'c, EmptyCtx>> {
        let aws_workflow = &self.aws_workflow;
        let template_file = aws_workflow.project().template_file_abs()?;
        let template_pkg_file = aws_workflow.template_pkg_file()?;
        let deploy_bucket_name = aws_workflow.bucket_deploy_name()?;

        self.set_region()?;
        self.software.args(&[
            "cloudformation",
            "package",
            "--template-file",
            template_file.to_string_lossy().trim(),
            "--s3-bucket",
            deploy_bucket_name.as_ref(),
            "--output-template-file",
            template_pkg_file.to_string_lossy().trim(),
        ]);
        Ok(self.software.runner(EmptyCtx {}))
    }

    pub fn cloudformation_deploy(mut self) -> Result<Runner<'c, EmptyCtx>> {
        let aws_workflow = &self.aws_workflow;
        let template_pkg_file = aws_workflow.template_pkg_file()?;
        let stack_name = aws_workflow.stack_name()?;

        let mut capabilities = Capabilities::new();
        if aws_workflow
            .env()
            .is_set(ENV_AWS_CAPABILITY_NAMED_IAM, "true")
        {
            capabilities.add(Capability::CAPABILITY_NAMED_IAM);
        }

        if aws_workflow.env().is_set(ENV_AWS_CAPABILITY_IAM, "true") {
            capabilities.add(Capability::CAPABILITY_IAM);
        }

        self.set_region()?;
        self.software.args(&[
            "cloudformation",
            "deploy",
            "--template-file",
            template_pkg_file.to_string_lossy().trim(),
            "--stack-name",
            stack_name.as_ref(),
        ]);

        let parameter_overrides = ParameterOverrides::new(self.aws_workflow.env(), &stack_name);
        self.software.args(parameter_overrides.args().as_slice());

        if let Some(capabilities) = capabilities.to_strings() {
            self.software.arg("--capabilities");
            self.software.args(capabilities);
        }
        Ok(self.software.runner(EmptyCtx {}))
    }

    pub fn s3_bucket_exists(mut self) -> Result<Runner<'c, EmptyCtx>> {
        let bucket_name = self.aws_workflow.bucket_deploy_name()?;
        self.software.args(&["s3api", "head-bucket"]);
        self.software.args(&["--bucket", bucket_name.as_ref()]);
        Ok(self.software.runner(EmptyCtx {}))
    }

    pub fn s3_create_bucket(mut self) -> Result<Runner<'c, EmptyCtx>> {
        let bucket_name = self.aws_workflow.bucket_deploy_name()?;
        self.set_region()?;
        self.software
            .args(&["s3", "mb", format!("s3://{}", bucket_name).as_str()]);
        Ok(self.software.runner(EmptyCtx {}))
    }

    pub fn s3_bucket_location(mut self) -> Result<Runner<'c, AwsCtxS3BucketLocation>> {
        self.set_region()?;

        self.software.args(&[
            "s3api",
            "get-bucket-location",
            "--bucket",
            self.aws_workflow.bucket_deploy_name()?.as_str(),
        ]);

        Ok(self.software.runner(AwsCtxS3BucketLocation {
            region: self.aws_workflow.region()?,
        }))
    }
}

pub struct AwsCtxS3BucketLocation {
    pub region: String,
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::workflow::{AwsWorkflow, ENV_AWS_REGION, ENV_AWS_S3_BUCKET_DEPLOY};
    use crate::exec::ExecCtx;
    use crate::project::Projects;
    use short_env::Env;

    fn env() -> Env {
        let mut env = Env::new();
        env.add(ENV_AWS_S3_BUCKET_DEPLOY, "test_deploy_bucket");
        env.add(ENV_AWS_REGION, "test-region");
        env.set_name("env_test");
        env
    }

    fn exec_ctx() -> ExecCtx {
        let mut exec_ctx = ExecCtx::new();
        exec_ctx.set_dry_run(true);
        exec_ctx
    }

    #[test]
    fn cloudformation_package() {
        let projects = Projects::fake();
        let project = projects.current_project().unwrap();
        let env = env();
        let exec_ctx = exec_ctx();
        let aws_workflow = AwsWorkflow::new(&project, &env, &exec_ctx);
        let runner = aws_workflow
            .cli_aws()
            .unwrap()
            .cloudformation_package()
            .unwrap();
        assert_eq!(format!("{}",runner),"aws --region test-region cloudformation package --template-file /path/to/local/./project_test.tpl --s3-bucket test_deploy_bucket --output-template-file /path/to/local/project_test.pkg.tpl");
    }

    #[test]
    fn cloudformation_deploy() {
        let projects = Projects::fake();
        let project = projects.current_project().unwrap();
        let env = env();
        let exec_ctx = exec_ctx();
        let aws_workflow = AwsWorkflow::new(&project, &env, &exec_ctx);
        let runner = aws_workflow
            .cli_aws()
            .unwrap()
            .cloudformation_deploy()
            .unwrap();
        assert_eq!(format!("{}",runner),"aws --region test-region cloudformation deploy --template-file /path/to/local/project_test.pkg.tpl --stack-name project_test-env_test --parameter-overrides StackName=project_test-env_test");
    }

    #[test]
    fn s3_bucket_exists() {
        let projects = Projects::fake();
        let project = projects.current_project().unwrap();
        let env = env();
        let exec_ctx = exec_ctx();
        let aws_workflow = AwsWorkflow::new(&project, &env, &exec_ctx);
        let runner = aws_workflow.cli_aws().unwrap().s3_bucket_exists().unwrap();

        assert_eq!(
            format!("{}", runner),
            "aws s3api head-bucket --bucket test_deploy_bucket"
        );

        let aws_workflow = AwsWorkflow::new(&project, &env, &exec_ctx);
        let runner = aws_workflow.cli_aws().unwrap().s3_create_bucket().unwrap();
        assert_eq!(
            format!("{}", runner),
            "aws --region test-region s3 mb s3://test_deploy_bucket"
        );
    }
}
