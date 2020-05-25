use crate::cli::cfg::get_cfg;
use anyhow::Result;
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
        table.add_row(Row::new(vec![TableCell::new(name)]));
    }
    println!("{}", table.render());
    Ok(())
}
