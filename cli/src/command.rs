use crate::helper::{get_entry_abs, reach_directories};
use clap::ArgMatches;

use crate::{BIN_NAME, VERSION};
use d4d::env::get;
use d4d::exec::aws::aws_output::{AwsOutputS3BucketLocation, AwsOutputS3Exists};
use d4d::exec::aws::workflow::AwsWorkflow;
use d4d::exec::ExecCtx;
use d4d::project::Projects;
use indicatif::{ProgressBar, ProgressStyle};
use promptly::prompt_default;
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

pub fn run_command(exec_ctx: &ExecCtx, projects: &Projects) -> Result<()> {
    let project = projects.current_project()?;
    let env = projects.current_env()?;
    let env = get(&project, &env)?;
    let runner = AwsWorkflow::new(&project, &env, &exec_ctx)
        .cli_aws()?
        .s3_bucket_exists()?;
    if let Some(output) = runner.run2()? {
        let output: Result<AwsOutputS3Exists> = output.into();
        let s3exit = output?;

        if !s3exit.is_exists() {
            if !prompt_default(
                "s3 deployment bucket is missing : do you want to create it ?",
                true,
            )? {
                return Err(Error::from("we have to create one before deployment"));
            }
            let runner = AwsWorkflow::new(&project, &env, &exec_ctx)
                .cli_aws()?
                .s3_create_bucket()?;
            runner.run()?;
        }
    }

    let runner = AwsWorkflow::new(&project, &env, &exec_ctx)
        .cli_aws()?
        .s3_bucket_location()?;

    if let Some(output) = runner.run2()? {
        let output: Result<AwsOutputS3BucketLocation> = output.into();
        output?;
    }

    let runner = AwsWorkflow::new(&project, &env, &exec_ctx)
        .cli_aws()?
        .cloudformation_package()?;
    runner.run()?;
    let runner = AwsWorkflow::new(&project, &env, &exec_ctx)
        .cli_aws()?
        .cloudformation_deploy()?;
    runner.spawn()?;
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
                project.template_file_rel()?.to_string_lossy()
            );
        } else {
            return Err(Error::from(
                "incorrect arguments : project name or path to yaml is missing",
            ));
        }
    }
    Ok(())
}

use std::thread;
use std::time::Duration;

pub fn demo_command() {
    let p = ProgressBar::new(1);
    p.set_style(ProgressStyle::default_spinner().template(" [{spinner:.cyan/blue}] {msg}"));
    p.enable_steady_tick(10);
    p.set_message("loading ...");
    thread::sleep(Duration::from_secs(1));
    p.set_style(ProgressStyle::default_spinner().template(" [x] {msg:.green}"));
    p.finish_with_message("ok");
    println!("{} - {}", BIN_NAME, VERSION);
}
