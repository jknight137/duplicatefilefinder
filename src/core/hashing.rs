use sha2::{Sha256, Digest};
use rayon::prelude::*;
use std::{collections::HashMap, fs, path::PathBuf};

pub fn compute_hash(path: &PathBuf) -> Option<String> {
    fs::File::open(path).ok().map(|mut file| {
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher).unwrap();
        format!("{:x}", hasher.finalize())
    })
}

pub fn find_duplicates(files: &[PathBuf]) -> HashMap<String, Vec<PathBuf>> {
    files.par_iter()
        .filter_map(|path| {
            compute_hash(path).map(|hash| (hash, path.clone()))
        })
        .fold(
            || HashMap::<String, Vec<PathBuf>>::new(),
            |mut acc, (hash, path)| {
                acc.entry(hash).or_default().push(path);
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut acc, map| {
                for (hash, paths) in map {
                    acc.entry(hash).or_default().extend(paths);
                }
                acc
            },
        )
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .collect()
}
