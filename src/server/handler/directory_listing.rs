use std::{fs::{self, ReadDir}, io, path::Path, time::SystemTime};
use serde::Serialize;

#[derive(Serialize)]
pub(super) struct FileSystem {
    items: Vec<Item>
}

impl FileSystem {
    fn get_content<P: AsRef<Path>>(path: P) -> Result<ReadDir, io::Error> {
        fs::read_dir(path)
    }

    pub(super) fn listing<P: AsRef<Path>>(path: P) -> Result<Vec<Item>, io::Error> {
        let mut items = Vec::new();
        let content = Self::get_content(path)?;

        for entry in content {
            let entry = entry?;
            let name = entry.file_name().into_string().unwrap();
            let metadata = entry.metadata()?;
            let size = metadata.len();
            let modified_at = metadata.modified()?;
            let item = Item { name, size, modified_at };

            items.push(item);
        }

        Ok(items)
    }
}

#[derive(Serialize)]
pub(super) struct Item {
    name: String,
    size: u64,
    modified_at: SystemTime
}

// impl Item {
//     fn get_name(&self) -> &str { &self.name }
//     fn get_link(&self) -> &Link { &self.link }
//     fn get_size(&self) -> u64 { self.size }
//     fn get_modified(&self) -> &SystemTime { &self.modified }
// }
