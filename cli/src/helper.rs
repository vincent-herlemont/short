use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use short_core::exec::output::Output;
use short_core::exec::Runner;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use utils::error::Error;
use utils::result::Result;

/// return (current_dir,home_dir)
pub fn reach_directories() -> Result<(PathBuf, PathBuf)> {
    match (current_dir(), dirs::home_dir()) {
        (Ok(current_dir), Some(home_dir)) => Ok((current_dir, home_dir)),
        (Err(err), _) => Err(Error::wrap(
            "fail to reach current directory",
            Error::from(err),
        )),
        (_, None) => Err(Error::new(
            "fail to reach home directory, please check your $HOME (linux,osx) or FOLDERID_Profile (windows)",
        )),
    }
}

// return absolute path of entry and test is this entry exists
pub fn get_entry_abs<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let (current_dir, _) = reach_directories()?;
    current_dir.join(&path).canonicalize().map_err(|err| {
        Error::wrap(
            format!("{} not found", path.as_ref().to_string_lossy()),
            Error::from(err),
        )
    })
}

pub fn run_progress<C, M>(
    runner: Runner<C>,
    start_message: M,
    finish_message: M,
) -> Result<Option<Output<C>>>
where
    M: AsRef<str>,
{
    if runner.exec_ctx().dry_run() || runner.exec_ctx().verbose() {
        println!("{}", runner);
    }
    let p = Arc::new(ProgressBar::new(1));
    p.enable_steady_tick(10);
    p.set_message(start_message.as_ref());
    p.set_style(ProgressStyle::default_spinner().template(" [{spinner:.cyan/blue}] {wide_msg}"));

    let run_p = p.clone();
    let runner = runner.set_display(Box::new(move |line: String| -> () {
        if !line.is_empty() {
            run_p.set_message(line.as_str())
        }
    }));

    let output = runner.spawn2()?;
    p.set_style(ProgressStyle::default_spinner().template(" [x] {wide_msg:.green}"));
    p.finish_with_message(finish_message.as_ref());

    if let Some(output) = &output {
        if !&output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr.as_ref())
                .as_ref()
                .red();
            println!("{}", stderr);
        }
    }

    Ok(output)
}
