use crate::exec::aws::aws::Aws;
use crate::exec::aws::capabilities::Capabilities;
use crate::exec::{ExecCtx, Runner};

use crate::project::Project;
use env::Env;
use std::path::PathBuf;
use utils::error::Error;
use utils::result::Result;

const AWS_S3_BUCKET_DEPLOY: &'static str = "AWS_S3_BUCKET_DEPLOY";

#[derive(Debug)]
pub struct AwsWorkflow<'a> {
    project: &'a Project<'a>,
    aws: Aws<'a>,
}

impl<'a> AwsWorkflow<'a> {
    pub fn new(project: &'a Project, exec_ctx: &'a ExecCtx) -> Result<Self> {
        Ok(Self {
            project,
            aws: Aws::new(exec_ctx)?,
        })
    }

    pub fn fake(project: &'a Project, exec_ctx: &'a ExecCtx) -> Self {
        Self {
            project,
            aws: Aws::fake(exec_ctx),
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

    pub fn package(self, env: &Env) -> Result<Runner> {
        let template_file = self.project.template_file()?;
        let template_pkg_file = self.template_pkg_file()?;
        let (_, deploy_bucket_name) = env.get(AWS_S3_BUCKET_DEPLOY).map_err(|err| {
            Error::wrap(
                format!("fail to package project {}", self.project.name()),
                Error::from(err),
            )
        })?;
        Ok(self.aws.cli_cloudformation_package(
            template_file,
            deploy_bucket_name,
            template_pkg_file,
        ))
    }

    pub fn deploy(self, env: &Env) -> Result<Runner> {
        let template_pkg_file = self.template_pkg_file()?;
        let stack_name = self.stack_name(env)?;
        Ok(self.aws.cli_cloudformation_deploy(
            template_pkg_file,
            stack_name,
            Capabilities::new(&[]),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::workflow::{AwsWorkflow, AWS_S3_BUCKET_DEPLOY};
    use crate::exec::ExecCtx;
    use crate::project::{Project, Projects};
    use env::Env;
    use std::path::PathBuf;

    fn aws_workflow_env<'a>(project: &'a Project, exec_ctx: &'a ExecCtx) -> (AwsWorkflow<'a>, Env) {
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
        assert_eq!(format!("{}",runner),"aws --region test-region cloudformation package --template-file /path/to/local/./project_test.tpl --s3-bucket test_deploy_bucket --output-template-file /path/to/local/project_test.pkg.tpl");
    }

    #[test]
    fn deploy() {
        let projects = Projects::fake();
        let exec_ctx = ExecCtx::new();
        let project = projects.current_project().unwrap();
        let (aws_workflow, env) = aws_workflow_env(&project, &exec_ctx);
        let runner = aws_workflow.deploy(&env).unwrap();
        assert_eq!(format!("{}",runner),"aws --region test-region cloudformation deploy --template-file /path/to/local/project_test.pkg.tpl --stack-name project_test-env_test")
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
}
