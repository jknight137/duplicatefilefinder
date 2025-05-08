use walkdir::WalkDir;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

/// Scans folders and reports files > min_size while supporting cancellation and live counting.
pub fn scan_files_interruptible(
    base_paths: &[PathBuf],
    min_size: u64,
    cancel: Arc<AtomicBool>,
    live_count: Arc<Mutex<usize>>,
) -> Vec<PathBuf> {
    let mut files = vec![];

    for base_path in base_paths {
        println!("📂 Scanning folder: {}", base_path.display());

        for entry in WalkDir::new(base_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if cancel.load(Ordering::Relaxed) {
                println!("🛑 Scan cancelled.");
                return files;
            }

            let path = entry.path();
            match entry.metadata() {
                Ok(metadata) => {
                    let size = metadata.len();
                    if size >= min_size {
                        println!("  ✅ Found file: {} ({} bytes)", path.display(), size);
                        files.push(path.to_path_buf());

                        let mut count = live_count.lock().unwrap();
                        *count += 1;
                    }
                }
                Err(_) => {}
            }
        }
    }

    files
}
