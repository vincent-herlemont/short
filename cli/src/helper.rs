use std::env::current_dir;
use std::path::{Path, PathBuf};
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
