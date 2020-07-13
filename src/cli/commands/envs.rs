use anyhow::Result;
use clap::ArgMatches;

use prettytable::format;
use prettytable::Attr;
use prettytable::{Cell, Row, Table};

use crate::cli::cfg::get_cfg;
use crate::cli::commands::sync::{sync_workflow, SyncSettings};
use crate::cli::settings::get_settings;
use crate::env_file::Env;
use crate::utils::colorize::is_cli_colorized;
use prettytable::color::BLUE;

pub fn envs(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app, &cfg);

    let setup_name = settings.setup()?;
    let setup = cfg.current_setup(setup_name)?;

    let envs: Vec<_> = setup.envs().into_iter().filter_map(|r| r.ok()).collect();
    let recent_env = Env::recent(&envs)?;
    let sync_settings = SyncSettings::new(app);
    let mut envs = sync_workflow(recent_env, envs, sync_settings)?;
    envs.sort();
    let envs = envs;

    if envs.is_empty() {
        println!("there is no env");
        return Ok(());
    }

    let is_current_env = |env: &Env| {
        if let Ok(current_env) = settings.env() {
            if let Ok(env_name) = env.name() {
                if *current_env == env_name {
                    return true;
                }
            }
        }
        false
    };

    let env_ref = envs.get(0).map(|env| env.clone()).unwrap();

    // Display matrix
    let mut render_table = Table::new();
    render_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    let mut title: Row = Row::new(vec![Cell::new("")]);
    let mut current_env_column_index = 0;
    for (i, env) in envs.iter().enumerate() {
        let mut cell = Cell::new(env.name()?.as_str()).with_style(Attr::Bold);
        if is_current_env(env) {
            cell = cell.with_style(Attr::ForegroundColor(BLUE));
            current_env_column_index = i;
        }
        title.add_cell(cell);
    }
    render_table.add_row(title);

    for var_ref in env_ref.iter() {
        let mut line = Row::new(vec![]);
        line.add_cell(Cell::new(var_ref.name()).with_style(Attr::Bold));
        for (i, env) in envs.iter().enumerate() {
            if let Ok(var) = env.get(var_ref.name()) {
                let mut cell = Cell::new(var.value());
                if i == current_env_column_index {
                    cell = cell.with_style(Attr::ForegroundColor(BLUE));
                }
                line.add_cell(cell);
            }
        }
        render_table.add_row(line);
    }

    if is_cli_colorized() {
        render_table.print_tty(true);
    } else {
        render_table.printstd();
    }

    Ok(())
}
