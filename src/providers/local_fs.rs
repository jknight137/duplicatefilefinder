use crate::core::models::{FileEntry, FileSource};
use walkdir::WalkDir;
use std::path::PathBuf;

pub fn get_local_files(path: &PathBuf, min_size: u64) -> Vec<FileEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.metadata().ok().map(|metadata| (entry, metadata))
        })
        .filter(|(_, metadata)| metadata.len() >= min_size)
        .map(|(entry, metadata)| FileEntry {
            path: entry.path().to_path_buf(),
            size: metadata.len(),
            hash: None,
            source: FileSource::Local,
        })
        .collect()
}
