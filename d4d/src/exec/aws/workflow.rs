
use crate::exec::aws::cli_aws::CliAws;
use crate::exec::{ExecCtx};

use crate::project::Project;
use env::Env;
use std::path::PathBuf;
use utils::error::Error;
use utils::result::Result;

/// Required environment variables
pub const ENV_AWS_S3_BUCKET_DEPLOY: &'static str = "AWS_S3_BUCKET_DEPLOY";
pub const ENV_AWS_REGION: &'static str = "AWS_REGION";
pub const ENV_AWS_CAPABILITY_NAMED_IAM: &'static str = "AWS_CAPABILITY_NAMED_IAM";
pub const ENV_AWS_CAPABILITY_IAM: &'static str = "AWS_CAPABILITY_IAM";

#[derive(Debug)]
pub struct AwsWorkflow<'p, 'e, 'c> {
    project: &'p Project<'p>,
    env: &'e Env,
    exec_ctx: &'c ExecCtx,
}

impl<'p, 'e, 'c> AwsWorkflow<'p, 'e, 'c> {
    pub fn new(project: &'p Project, env: &'e Env, exec_ctx: &'c ExecCtx) -> Self {
        Self {
            project,
            env,
            exec_ctx,
        }
    }

    pub fn exec_ctx(&self) -> &'c ExecCtx {
        self.exec_ctx
    }

    pub fn env(&self) -> &'e Env {
        self.env
    }

    pub fn project(&self) -> &'p Project {
        self.project
    }

    pub fn cli_aws(self) -> Result<CliAws<'p, 'e, 'c>> {
        CliAws::new(self)
    }

    pub fn template_pkg_file(&self) -> Result<PathBuf> {
        let mut template_file = self.project.template_file_abs()?;
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

    pub fn stack_name(&self) -> Result<String> {
        let project_name = self.project.name();
        let project_env = self.env.name()?;
        Ok(format!("{}-{}", project_name, project_env))
    }

    pub fn s3_bucket_name(&self) -> Result<String> {
        let (_, deploy_bucket_name) = self.env.get(ENV_AWS_S3_BUCKET_DEPLOY).map_err(|err| {
            Error::wrap(
                format!("fail to package project {}", self.project.name()),
                Error::from(err),
            )
        })?;
        Ok(deploy_bucket_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::workflow::{AwsWorkflow, ENV_AWS_REGION, ENV_AWS_S3_BUCKET_DEPLOY};
    use crate::exec::ExecCtx;
    use crate::project::Projects;
    use env::Env;
    use std::path::PathBuf;

    fn env() -> Env {
        let mut env = Env::new();
        env.add(ENV_AWS_S3_BUCKET_DEPLOY, "test_deploy_bucket");
        env.add(ENV_AWS_REGION, "test-region");
        env.set_name("env_test");
        env
    }

    fn exec_ctx() -> ExecCtx {
        ExecCtx::new().set_dry_run(true)
    }

    #[test]
    fn stack_name() {
        let projects = Projects::fake();
        let project = projects.current_project().unwrap();
        let env = env();
        let exec_ctx = exec_ctx();
        let aws_workflow = AwsWorkflow::new(&project, &env, &exec_ctx);

        let stack_name = aws_workflow.stack_name().unwrap();
        assert_eq!(stack_name, "project_test-env_test");
    }

    #[test]
    fn template_pkg_file() {
        let projects = Projects::fake();
        let project = projects.current_project().unwrap();
        let env = env();
        let exec_ctx = exec_ctx();
        let aws_workflow = AwsWorkflow::new(&project, &env, &exec_ctx);

        let template_pkg_file = aws_workflow.template_pkg_file().unwrap();
        assert_eq!(
            template_pkg_file,
            PathBuf::from("/path/to/local/project_test.pkg.tpl")
        );
    }
}
