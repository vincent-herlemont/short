use crate::cfg::LocalSetupCfg;
use crate::cli::cfg::get_cfg;

use crate::cli::terminal::message::success;

use crate::run_file::File;
use anyhow::{Result};
use clap::ArgMatches;

use std::path::PathBuf;

pub fn new(app: &ArgMatches) -> Result<()> {
    let mut cfg = get_cfg()?;
    let setup_name = app.value_of("setup_name").unwrap();
    let setup_file = app.value_of("file").unwrap();
    let setup_shebang = app.value_of("shebang").unwrap();

    let setup_file = PathBuf::from(setup_file);

    let local_setup_cfg = LocalSetupCfg::new(setup_name.into(), setup_file.clone());
    let array_vars = local_setup_cfg.array_vars();
    let vars = local_setup_cfg.vars();

    cfg.add_local_setup_cfg(local_setup_cfg);
    cfg.sync_local_to_global()?;
    cfg.save()?;

    let mut file = File::new(setup_file.clone(), setup_shebang.to_string());
    {
        let array_vars = array_vars.as_ref().map(|r| r.borrow());
        let vars = vars.as_ref().map(|r| r.borrow());
        file.generate(array_vars, vars)?;
    }
    file.save()?;

    success(format!("new setup {}", setup_name).as_str());

    Ok(())
}
