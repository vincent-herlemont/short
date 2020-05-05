use fs_extra::dir::create_all;
use fs_extra::file::read_to_string;
use fs_extra::file::write_all;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use tempdir::TempDir;
use walkdir::WalkDir;

#[derive(Debug)]
struct CliIntegrationEnvironment {
    label: String,
    tmp_dir: TempDir,
    entries: HashMap<PathBuf, Option<String>>,
}

impl CliIntegrationEnvironment {
    fn new<L>(label: L) -> Self
    where
        L: AsRef<str>,
    {
        let label = label.as_ref().to_string();
        let tmp_dir = TempDir::new(&label).expect("fail to create tmp directory");
        Self {
            label,
            tmp_dir,
            entries: HashMap::new(),
        }
    }

    fn add_file<P, C>(&mut self, path: P, content: C)
    where
        P: AsRef<Path>,
        C: AsRef<str>,
    {
        self.entries.insert(
            path.as_ref().to_path_buf(),
            Some(content.as_ref().to_string()),
        );
    }

    fn read_file<P>(&self, path: P) -> String
    where
        P: AsRef<Path>,
    {
        let path = self.tmp_dir.path().join(path.as_ref());
        read_to_string(&path).expect(format!("fail to read file {:?}", path).as_str())
    }

    fn add_dir<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        self.entries.insert(path.as_ref().to_path_buf(), None);
    }

    fn setup(&self) {
        for (path, content) in self.entries.iter() {
            let path = self.tmp_dir.path().join(path);
            if let Some(content) = content {
                if let Some(path) = path.parent() {
                    create_all(path, false)
                        .expect(format!("fail to create directory {:?}", path).as_str())
                }
                write_all(path, content).expect("fail to create file");
            } else {
                create_all(&path, false)
                    .expect(format!("fail to create directory {:?}", path).as_str())
            }
        }
    }

    fn tree(&self) -> Vec<PathBuf> {
        let mut tree: Vec<PathBuf> = WalkDir::new(self.tmp_dir.path())
            .into_iter()
            .filter_map(|dir_entry| {
                if let Ok(dir_entry) = dir_entry {
                    if let Ok(dir_entry) = dir_entry.path().strip_prefix(self.tmp_dir.path()) {
                        return Some(dir_entry.to_path_buf());
                    }
                }
                None
            })
            .collect();
        tree.sort();
        tree
    }
}

impl Display for CliIntegrationEnvironment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for e in self.tree() {
            writeln!(f, "{}", e.to_string_lossy())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::integration_test::CliIntegrationEnvironment;
    use predicates::prelude::Predicate;
    use predicates::str::contains;

    #[test]
    fn simple() {
        let mut e = CliIntegrationEnvironment::new("test");
        e.add_file("file1", "test 1");
        e.add_file("dir/file2", "test 2");
        e.add_dir("emptry_dir");
        e.setup();
        let display = e.to_string();
        assert!(contains("file1").eval(display.as_str()));
        assert!(contains("dir/file2").eval(display.as_str()));
        assert!(contains("emptry_dir").eval(display.as_str()));

        assert!(contains("test 1").eval(e.read_file("file1").as_str()));
    }
}
