use crate::project::Projects;
use std::path::PathBuf;
use utils::error::Error;
use utils::result::Result;

#[derive(Debug)]
pub struct AwsWorkflow<'a> {
    projects: &'a Projects,
}

impl<'a> AwsWorkflow<'a> {
    pub fn fake(projects: &'a Projects) -> Self {
        Self { projects: projects }
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
}

#[cfg(test)]
mod tests {
    use crate::exec::aws::workflow::AwsWorkflow;
    use crate::project::Projects;
    use std::path::PathBuf;

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
