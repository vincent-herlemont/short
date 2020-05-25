use crate::cli::cfg::get_cfg;
use anyhow::Result;

pub fn ls() -> Result<()> {
    let cfg = get_cfg()?;
    Ok(())
}
