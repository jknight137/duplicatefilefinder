use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub hash: Option<String>,
    pub source: FileSource,
}

#[derive(Debug, Clone)]
pub enum FileSource {
    Local,
    GoogleDrive,
    ICloud,
    OneDrive,
    Dropbox,
}

#[derive(Debug)]
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<FileEntry>,
}
