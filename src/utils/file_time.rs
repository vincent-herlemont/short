use filetime::FileTime;
use std::fs;
use std::path::Path;

pub type ModificationTime = FileTime;
pub type CreateTime = FileTime;

pub fn file_time(file: &Path) -> (ModificationTime, Option<CreateTime>) {
    let metadata = fs::metadata(file).unwrap();
    (
        FileTime::from_last_modification_time(&metadata),
        FileTime::from_creation_time(&metadata),
    )
}

pub fn create_time(file: &Path) -> CreateTime {
    match file_time(file) {
        (_, Some(create_time)) => create_time,
        (modification_time, None) => modification_time,
    }
}

pub fn modification_time(file: &Path) -> ModificationTime {
    match file_time(file) {
        (modification_time, _) => modification_time,
    }
}
