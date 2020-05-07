use crate::cfg::{find_local_cfg, CliCfg};
use anyhow::Result;
use short_core::LocalCfg;

pub fn ls() -> Result<()> {
    let local_cfg = find_local_cfg(None, "short.yml".to_string())?;
    let local_cfg: LocalCfg = CliCfg::from_file(local_cfg)?;
    dbg!(local_cfg);
    Ok(())
}
