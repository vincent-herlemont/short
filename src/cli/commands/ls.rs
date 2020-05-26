use crate::cli::cfg::get_cfg;
use crate::env_file::Env;
use anyhow::Result;
use log::*;
use term_table::row::Row;
use term_table::table_cell::TableCell;
use term_table::{Table, TableStyle};

pub fn ls() -> Result<()> {
    let mut cfg = get_cfg()?;
    cfg.sync_local_to_global()?;
    let cfg = cfg;
    let local_setups = cfg.local_setups()?;
    let mut table = Table::new();
    table.style = TableStyle::blank();
    table.has_bottom_boarder = false;
    table.has_top_boarder = false;
    table.separate_rows = false;

    for local_setup in local_setups {
        let name = local_setup.name()?;
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
                TableCell::new("<none>"),
                TableCell::new(&name),
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

                table.add_row(Row::new(vec![
                    TableCell::new(&env_name),
                    TableCell::new(&name),
                ]));
            }
        }
    }
    println!("{}", table.render());
    Ok(())
}
