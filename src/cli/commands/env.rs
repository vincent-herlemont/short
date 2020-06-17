use crate::cli::cfg::get_cfg;
use crate::cli::commands::sync::{sync_workflow, SyncSettings};
use crate::cli::settings::get_settings;
use crate::env_file::Env;
use crate::run_file::{generate_env_vars, run_as_stream};
use anyhow::{Context, Result};
use clap::ArgMatches;
use term_table::row::Row;
use term_table::table_cell::TableCell;

pub fn env(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;

    let settings = get_settings(app, &cfg);

    let setup_name = settings.setup()?;
    let setup = cfg.current_setup(setup_name)?;

    let envs: Vec<_> = setup.envs().into_iter().filter_map(|r| r.ok()).collect();
    let recent_env = Env::recent(&envs)?;
    let sync_settings = SyncSettings::new(app);
    let envs = sync_workflow(recent_env, envs, sync_settings)?;

    if envs.is_empty() {
        println!("there is no env");
        return Ok(());
    }

    let env_ref = envs.get(0).map(|env| env.clone()).unwrap();

    let mut title: Vec<_> = envs.iter().map(|env| env.name().unwrap().clone()).collect();
    title.splice(0..0, vec!["".to_string()].into_iter());

    let mut table = vec![title];
    for var_ref in env_ref.iter() {
        let mut line = vec![];
        line.push(var_ref.name().clone());
        for env in &envs {
            let var = env.get(var_ref.name()).unwrap();
            line.push(var.value().clone());
        }
        table.push(line);
    }

    // Display matrix
    let mut render_table = term_table::Table::new();
    render_table.style = term_table::TableStyle::thin();

    for row in table {
        let cells: Vec<_> = row.iter().map(|cell| TableCell::new(cell)).collect();
        render_table.add_row(Row::new(cells));
    }
    println!("{}", render_table.render());

    Ok(())
}
