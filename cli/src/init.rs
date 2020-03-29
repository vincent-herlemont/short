use crate::helper::reach_directories;
use clap::ArgMatches;
use d4d::exec::ExecCtx;
use d4d::project::{CurrentProject, Projects};


use utils::result::Result;

/// Return the execution context.
/// It's control the execution behavior of externals commands.
pub fn init_exec_ctx(app: &ArgMatches) -> ExecCtx {
    let exec_ctx = ExecCtx::new();
    if app.is_present("dry-run") {
        exec_ctx.set_dry_run(true)
    } else {
        exec_ctx
    }
}

/// Return global project object, it's use as context repository for the most command.
pub fn init_projects(args: &ArgMatches) -> Result<Projects> {
    let (current_dir, home_dir) = reach_directories()?;

    let mut projects = Projects::load(current_dir, home_dir)?;

    if let Some(project_name) = args.value_of_lossy("project") {
        let current_project = CurrentProject::new(project_name);
        projects.set_temporary_current_project(current_project.clone());

        if let Some(env) = args.value_of_lossy("env") {
            let current_project = current_project.set_env(env);
            projects.set_temporary_current_project(current_project);
        }
    }

    Ok(projects)
}
