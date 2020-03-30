use crate::exec::aws::aws::Aws;
use crate::exec::aws::capabilities::{Capabilities, Capability};
use crate::exec::{ExecCtx, Runner};

use crate::project::Project;
use env::Env;
use std::path::PathBuf;
use utils::error::Error;
use utils::result::Result;

const AWS_S3_BUCKET_DEPLOY: &'static str = "AWS_S3_BUCKET_DEPLOY";

#[derive(Debug)]
pub struct AwsWorkflow<'a, 'b> {
    project: &'a Project<'a>,
    aws: Aws<'b, 'a>,
}

impl<'a, 'b> AwsWorkflow<'a, 'b> {
    pub fn new(project: &'a Project, exec_ctx: &'b ExecCtx) -> Result<Self> {
        let aws_cfg = project.aws()?;
        Ok(Self {
            project,
            aws: Aws::new(aws_cfg, exec_ctx)?,
        })
    }

    pub fn fake(project: &'a Project, exec_ctx: &'b ExecCtx) -> Self {
        let aws_cfg = project.aws().unwrap();
        Self {
            project,
            aws: Aws::fake(aws_cfg, exec_ctx),
        }
    }

    fn template_pkg_file(&self) -> Result<PathBuf> {
        let mut template_file = self.project.template_file()?;
        let file_name = template_file
            .file_name()
            .ok_or(Error::new(format!(
                "fail to found template file name {} of {}",
                self.project.name(),
                template_file.to_string_lossy()
            )))?
            .to_str()
            .ok_or(Error::new(format!(
                "forbidden no UTF-8 char on template file name {} of {}",
                self.project.name(),
                template_file.to_string_lossy()
            )))?
            .to_string();

        let mut chunks: Vec<_> = file_name.rsplitn(2, '.').collect();
        chunks.insert(1, "pkg");
        chunks.reverse();
        template_file.set_file_name(chunks.join("."));
        Ok(template_file)
    }

    fn stack_name(&self, env: &Env) -> Result<String> {
        let project_name = self.project.name();
        let project_env = env.name()?;
        Ok(format!("{}-{}", project_name, project_env))
    }

    fn s3_bucket_name(&self, env: &Env) -> Result<String> {
        let (_, deploy_bucket_name) = env.get(AWS_S3_BUCKET_DEPLOY).map_err(|err| {
            Error::wrap(
                format!("fail to package project {}", self.project.name()),
                Error::from(err),
            )
        })?;
        Ok(deploy_bucket_name)
    }

    pub fn package(self, env: &Env) -> Result<Runner<'b>> {
        let template_file = self.project.template_file()?;
        let template_pkg_file = self.template_pkg_file()?;
        let deploy_bucket_name = self.s3_bucket_name(&env)?;

        Ok(self.aws.cli_cloudformation_package(
            template_file,
            deploy_bucket_name,
            template_pkg_file,
        ))
    }

    pub fn deploy(self, env: &Env) -> Result<Runner<'b>> {
        let template_pkg_file = self.template_pkg_file()?;
        let stack_name = self.stack_name(env)?;

        let mut capabilities = Capabilities::new();
        if env.is_set("AWS_CAPABILITY_NAMED_IAM", "true") {
            capabilities.add(Capability::CAPABILITY_NAMED_IAM);
        }

        if env.is_set("AWS_CAPABILITY_IAM", "true") {
            capabilities.add(Capability::CAPABILITY_IAM);
        }

        Ok(self
            .aws
            .cli_cloudformation_deploy(template_pkg_file, stack_name, capabilities))
    }

    pub fn s3_exists(self, env: &Env) -> Result<Runner<'b>> {
        let bucket_name = self.s3_bucket_name(env)?;
        Ok(self.aws.cli_s3_bucket_exists(bucket_name))
    }

    pub fn s3_create_bucket(self, env: &Env) -> Result<Runner<'b>> {
        let bucket_name = self.s3_bucket_name(env)?;
        Ok(self.aws.cli_s3_create_bucket(bucket_name))
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::workflow::{AwsWorkflow, AWS_S3_BUCKET_DEPLOY};
    use crate::exec::ExecCtx;
    use crate::project::{Project, Projects};
    use env::Env;
    use std::path::PathBuf;

    fn aws_workflow_env<'a, 'b>(
        project: &'a Project,
        exec_ctx: &'b ExecCtx,
    ) -> (AwsWorkflow<'a, 'b>, Env) {
        let aws_workflow = AwsWorkflow::fake(&project, exec_ctx);
        let mut env = Env::new();
        env.add(AWS_S3_BUCKET_DEPLOY, "test_deploy_bucket");
        env.set_name("env_test");
        (aws_workflow, env)
    }

    #[test]
    fn package() {
        let projects = Projects::fake();
        let exec_ctx = ExecCtx::new();
        let project = projects.current_project().unwrap();
        let (aws_workflow, env) = aws_workflow_env(&project, &exec_ctx);
        let runner = aws_workflow.package(&env).unwrap();
        assert_eq!(format!("{}",runner),"aws --region us-east-1 cloudformation package --template-file /path/to/local/./project_test.tpl --s3-bucket test_deploy_bucket --output-template-file /path/to/local/project_test.pkg.tpl");
    }

    #[test]
    fn deploy() {
        let projects = Projects::fake();
        let exec_ctx = ExecCtx::new();
        let project = projects.current_project().unwrap();
        let (aws_workflow, env) = aws_workflow_env(&project, &exec_ctx);
        let runner = aws_workflow.deploy(&env).unwrap();
        assert_eq!(format!("{}",runner),"aws --region us-east-1 cloudformation deploy --template-file /path/to/local/project_test.pkg.tpl --stack-name project_test-env_test");

        // Test capabilities
        let (aws_workflow, mut env) = aws_workflow_env(&project, &exec_ctx);
        env.add("AWS_CAPABILITY_IAM", "true");
        env.add("AWS_CAPABILITY_NAMED_IAM", "true");
        let runner = aws_workflow.deploy(&env).unwrap();
        assert_eq!(format!("{}", runner),"aws --region us-east-1 cloudformation deploy --template-file /path/to/local/project_test.pkg.tpl --stack-name project_test-env_test --capabilities CAPABILITY_IAM CAPABILITY_NAMED_IAM");
    }

    #[test]
    fn stack_name() {
        let projects = Projects::fake();
        let exec_ctx = ExecCtx::new();
        let project = projects.current_project().unwrap();
        let (aws_workflow, env) = aws_workflow_env(&project, &exec_ctx);
        let stack_name = aws_workflow.stack_name(&env).unwrap();
        assert_eq!(stack_name, "project_test-env_test");
    }

    #[test]
    fn template_pkg_file() {
        let projects = Projects::fake();
        let exec_ctx = ExecCtx::new();
        let project = projects.current_project().unwrap();
        let (aws_workflow, _) = aws_workflow_env(&project, &exec_ctx);
        let template_pkg_file = aws_workflow.template_pkg_file().unwrap();
        assert_eq!(
            template_pkg_file,
            PathBuf::from("/path/to/local/project_test.pkg.tpl")
        );
    }

    #[test]
    fn s3_exists_and_create() {
        let projects = Projects::fake();
        let exec_ctx = ExecCtx::new();
        let project = projects.current_project().unwrap();

        let (aws_workflow, env) = aws_workflow_env(&project, &exec_ctx);
        let runner = aws_workflow.s3_exists(&env).unwrap();
        assert_eq!(
            format!("{}", runner),
            "aws s3api head-bucket --bucket test_deploy_bucket"
        );

        let (aws_workflow, env) = aws_workflow_env(&project, &exec_ctx);
        let runner = aws_workflow.s3_create_bucket(&env).unwrap();
        assert_eq!(
            format!("{}", runner),
            "aws --region us-east-1 s3api create-bucket --bucket test_deploy_bucket"
        );
    }
}
