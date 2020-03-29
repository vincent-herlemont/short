use crate::helper::{get_entry_abs, reach_directories};
use clap::ArgMatches;

use d4d::env::get;
use d4d::exec::aws::workflow::AwsWorkflow;
use d4d::exec::ExecCtx;
use d4d::project::Projects;
use utils::error::Error;
use utils::result::Result;

pub fn init_command() -> Result<()> {
    match reach_directories() {
        Ok((curent_dir, _)) => {
            Projects::init(&curent_dir)?;
            Ok(())
        }
        Err(err) => Err(Error::wrap("fail to init project", Error::from(err))),
    }
}

pub fn env_command(args: &ArgMatches, projects: &Projects) -> Result<()> {
    if let (Some(_), Some(vp), Some(ve)) = (
        args.values_of_lossy("check"),
        args.values_of_lossy("project"),
        args.values_of_lossy("env"),
    ) {
        if let (Some(project), Some(env)) = (vp.first(), ve.first()) {
            let project = projects.found(project)?;
            let env = get(&project, &env)?;
            println!("{}", &project);
            print!("{}", &env);
            return Ok(());
        } else {
            return Err(Error::new("fail to get env or project"));
        }
    }
    Ok(())
}

pub fn deploy_command(exec_ctx: &ExecCtx, projects: &Projects) -> Result<()> {
    let project = projects.current_project()?;
    let env = projects.current_env()?;
    let env = get(&project, &env)?;
    let runner = AwsWorkflow::new(&project, &exec_ctx)?.package(&env)?;
    runner.run()?;
    let runner = AwsWorkflow::new(&project, &exec_ctx)?.deploy(&env)?;
    runner.run()?;
    Ok(())
}

pub fn use_command(args: &ArgMatches, projects: &mut Projects) -> Result<()> {
    if let Some(args) = args.values_of_lossy("use_project") {
        if let (Some(project_name), Some(env_name)) = (args.get(0), args.get(1)) {
            projects.set_current_project_name(project_name);
            projects.set_current_env_name(env_name)?;
            projects.save()?;
        } else {
            return Err(Error::from(
                "incorrect arguments : project name or env name is missing",
            ));
        }
    }
    Ok(())
}

pub fn add_command(args: &ArgMatches, projects: &mut Projects) -> Result<()> {
    if let Some(args) = args.values_of_lossy("add_project") {
        if let (Some(project_name), Some(path_to_yaml)) = (args.get(0), args.get(1)) {
            let path_to_yaml = get_entry_abs(path_to_yaml)?;
            let project = projects.add(project_name, &path_to_yaml)?;
            println!("project name : {} ", project_name);
            println!(
                "path to template : {}",
                project.template_path_rel()?.to_string_lossy()
            );
        } else {
            return Err(Error::from(
                "incorrect arguments : project name or path to yaml is missing",
            ));
        }
    }
    Ok(())
}
