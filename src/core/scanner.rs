use walkdir::WalkDir;
use std::path::PathBuf;

pub fn scan_files(base_paths: &[PathBuf], min_size: u64) -> Vec<PathBuf> {
    let mut files = vec![];
    for base_path in base_paths {
        for entry in WalkDir::new(base_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let metadata = entry.metadata().unwrap();
            if metadata.len() >= min_size {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    files
}
