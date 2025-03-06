use {
    crate::utils::AppResult,
    serde::Serialize,
    std::{
        fs::{
            self,
            ReadDir,
        },
        io,
        path::Path,
        time::SystemTime,
    },
};
#[derive(serde::Serialize)]
pub(super) struct FileSystem {
    items: Vec<Item>,
}

#[derive(serde::Serialize)]
pub(super) struct Item {
    name:        String,
    link:        String,
    size:        u64,
    modified_at: i64, // Déjà converti en timestamp i64
}

impl FileSystem {
    fn get_content<P: AsRef<Path>>(path: P) -> Result<ReadDir, io::Error> { fs::read_dir(path) }

    pub(super) fn listing<P: AsRef<Path>>(path: P) -> AppResult<Vec<Item>> {
        let mut items = Vec::new();
        let content = Self::get_content(path)?;

        for entry in content {
            let entry = entry?;
            let full_path = entry.path();
            let name = entry
                .file_name()
                .into_string()
                .unwrap();
            let metadata = entry.metadata()?;
            let size = metadata.len();
            let modified_at = metadata.modified()?;
            let item = Item {
                name,
                size,
                link: full_path
                    .to_string_lossy()
                    .into_owned(),
                modified_at: modified_at
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
            };

            items.push(item);
        }

        Ok(items)
    }
}

// impl Item {
//     fn get_name(&self) -> &str { &self.name }
//     fn get_link(&self) -> &Link { &self.link }
//     fn get_size(&self) -> u64 { self.size }
//     fn get_modified(&self) -> &SystemTime { &self.modified }
// }
