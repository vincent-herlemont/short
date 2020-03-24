use crate::exec::aws::aws::Aws;
use crate::exec::aws::capabilities::Capabilities;
use crate::exec::Runner;
use crate::project::Projects;

use std::path::PathBuf;
use utils::error::Error;
use utils::result::Result;

#[derive(Debug)]
pub struct AwsWorkflow<'a> {
    projects: &'a Projects,
    aws: Aws,
}

impl<'a> AwsWorkflow<'a> {
    pub fn fake(projects: &'a Projects) -> Self {
        Self {
            projects: projects,
            aws: Aws::fake(),
        }
    }

    fn template_pkg_file(&self) -> Result<PathBuf> {
        let project = self.projects.current_project()?;
        let mut template_file = project.template_file()?;
        let file_name = template_file
            .file_name()
            .ok_or(Error::new(format!(
                "fail to found template file name {} of {}",
                project.name(),
                template_file.to_string_lossy()
            )))?
            .to_str()
            .ok_or(Error::new(format!(
                "forbidden no UTF-8 char on template file name {} of {}",
                project.name(),
                template_file.to_string_lossy()
            )))?
            .to_string();

        let mut chunks: Vec<_> = file_name.rsplitn(2, '.').collect();
        chunks.insert(1, "pkg");
        chunks.reverse();
        template_file.set_file_name(chunks.join("."));
        Ok(template_file)
    }

    fn stack_name(&self) -> Result<String> {
        let project_name = self.projects.current_project()?.name();
        let project_env = self.projects.current_env()?;
        Ok(format!("{}-{}", project_name, project_env))
    }

    fn package<B: AsRef<str>>(self, deploy_bucket_name: B) -> Result<Runner> {
        let project = self.projects.current_project()?;
        let template_file = project.template_file()?;
        let template_pkg_file = self.template_pkg_file()?;
        Ok(self.aws.cli_cloudformation_package(
            template_file,
            deploy_bucket_name,
            template_pkg_file,
        ))
    }

    fn deploy(self) -> Result<Runner> {
        let template_pkg_file = self.template_pkg_file()?;
        let stack_name = self.stack_name()?;
        Ok(self.aws.cli_cloudformation_deploy(
            template_pkg_file,
            stack_name,
            Capabilities::new(&[]),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::workflow::AwsWorkflow;
    use crate::project::Projects;
    use std::path::PathBuf;

    #[test]
    fn package() {
        let project = Projects::fake();
        let aws_workflow = AwsWorkflow::fake(&project);
        let runner = aws_workflow.package("test_deploy_bucket").unwrap();
        assert_eq!(format!("{}",runner),"aws --region test-region cloudformation package --template-file /path/to/local/./project_test.tpl --s3-bucket test_deploy_bucket --output-template-file /path/to/local/project_test.pkg.tpl");
    }

    #[test]
    fn deploy() {
        let project = Projects::fake();
        let aws_workflow = AwsWorkflow::fake(&project);
        let runner = aws_workflow.deploy().unwrap();
        assert_eq!(format!("{}",runner),"aws --region test-region cloudformation deploy --template-file /path/to/local/project_test.pkg.tpl --stack-name project_test-env_test")
    }

    #[test]
    fn stack_name() {
        let project = Projects::fake();
        let aws_workflow = AwsWorkflow::fake(&project);
        let stack_name = aws_workflow.stack_name().unwrap();
        assert_eq!(stack_name, "project_test-env_test");
    }

    #[test]
    fn template_pkg_file() {
        let project = Projects::fake();
        let aws_workflow = AwsWorkflow::fake(&project);
        let template_pkg_file = aws_workflow.template_pkg_file().unwrap();
        assert_eq!(
            template_pkg_file,
            PathBuf::from("/path/to/local/project_test.pkg.tpl")
        );
    }
}
