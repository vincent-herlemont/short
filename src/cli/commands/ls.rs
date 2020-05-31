use crate::cli::cfg::get_cfg;
use crate::cli::settings::get_settings;
use crate::cli::terminal::emoji;
use crate::env_file::Env;
use anyhow::Result;
use clap::ArgMatches;
use log::*;
use term_table::row::Row;
use term_table::table_cell::TableCell;
use term_table::{Table, TableStyle};

pub fn ls(app: &ArgMatches) -> Result<()> {
    let settings = get_settings(&app);

    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;
    let local_setups = cfg.current_setups()?;
    let mut table = Table::new();

    table.style = TableStyle::blank();
    table.has_bottom_boarder = false;
    table.has_top_boarder = false;
    table.separate_rows = false;

    for local_setup in local_setups {
        let setup_name = local_setup.name()?;
        let envs: Vec<Env> = local_setup
            .envs()
            .into_iter()
            .filter_map(|r| {
                if let Err(e) = &r {
                    error!("{}", e);
                }
                r.ok()
            })
            .collect();

        if envs.is_empty() {
            table.add_row(Row::new(vec![
                TableCell::new(""),
                TableCell::new("<none>"),
                TableCell::new(&setup_name),
            ]));
        } else {
            for env in envs {
                let env_name = match env.name() {
                    Ok(env_name) => env_name,
                    Err(e) => {
                        error!("{}", e);
                        continue;
                    }
                };

                let check = if let (Ok(setting_env), Ok(setting_setup)) =
                    (settings.env(), settings.setup())
                {
                    if setting_env == &env_name && setting_setup == &setup_name {
                        Some(emoji::CHECK)
                    } else {
                        None
                    }
                } else {
                    None
                };

                table.add_row(Row::new(vec![
                    TableCell::new(check.map_or("".to_string(), |s| s.to_string())),
                    TableCell::new(&env_name),
                    TableCell::new(&setup_name),
                ]));
            }
        }
    }
    println!("{}", table.render());
    Ok(())
}
