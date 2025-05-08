use sha2::{Sha256, Digest};
use rayon::prelude::*;
use std::{collections::HashMap, fs, path::PathBuf};

/// Compute a SHA256 hash of a file's contents.
/// Returns `None` if the file can't be opened or read.
pub fn compute_hash(path: &PathBuf) -> Option<String> {
    if let Ok(mut file) = fs::File::open(path) {
        let mut hasher = Sha256::new();
        if std::io::copy(&mut file, &mut hasher).is_ok() {
            return Some(format!("{:x}", hasher.finalize()));
        }
    }
    None
}

/// Return a HashMap of hash => Vec<PathBuf> for only duplicate files (length > 1).
pub fn find_duplicates(files: &[PathBuf]) -> HashMap<String, Vec<PathBuf>> {
    let hashed_files: Vec<(String, PathBuf)> = files
        .par_iter()
        .filter_map(|path| {
            compute_hash(path).map(|hash| (hash, path.clone()))
        })
        .collect();

    let mut map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for (hash, path) in hashed_files {
        map.entry(hash).or_default().push(path);
    }

    map.into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .collect()
}
