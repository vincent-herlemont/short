use fs_extra::dir::create_all;
use fs_extra::error::Result;
use fs_extra::file::write_all;
use std::path::Path;

pub fn write_all_dir<P, S>(path: P, content: S) -> Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let path = path.as_ref();
    let content = content.as_ref();
    if let Some(path) = path.parent() {
        if !path.exists() {
            create_all(path, false)?;
        }
    }
    write_all(path, content)?;
    Ok(())
}
