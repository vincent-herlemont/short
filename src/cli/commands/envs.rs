use anyhow::Result;
use clap::ArgMatches;
use term_table::row::Row;
use term_table::table_cell::TableCell;

use crate::cli::cfg::get_cfg;
use crate::cli::commands::sync::{sync_workflow, SyncSettings};
use crate::cli::settings::get_settings;
use crate::env_file::Env;

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

    let env_ref = envs.get(0).map(|env| env.clone()).unwrap();

    let mut title: Vec<String> = vec![];
    for env in &envs {
        title.push(env.name()?.clone());
    }
    title.splice(0..0, vec!["".to_string()].into_iter());

    let mut table = vec![title];
    for var_ref in env_ref.iter() {
        let mut line = vec![];
        line.push(var_ref.name().clone());
        for env in &envs {
            if let Ok(var) = env.get(var_ref.name()) {
                line.push(var.value().clone());
            }
        }
        table.push(line);
    }

    // Display matrix
    let mut render_table = term_table::Table::new();
    render_table.separate_rows = false;
    render_table.style = term_table::TableStyle::thin();

    for row in table {
        let cells: Vec<_> = row.iter().map(|cell| TableCell::new(cell)).collect();
        render_table.add_row(Row::new(cells));
    }
    println!("{}", render_table.render());

    Ok(())
}
