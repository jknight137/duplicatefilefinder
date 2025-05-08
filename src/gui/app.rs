use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;

use eframe::egui::{self, CentralPanel, Context};
use crate::core::{hashing, scanner};

#[derive(PartialEq)]
enum ScanState {
    Idle,
    Working,
    Done,
    Cancelled,
}

pub struct DuplicateApp {
    pub scan_path: String,
    pub scan_status: String,
    pub file_count: usize,
    pub deleted_files: usize,
    pub deleted_bytes: u64,
    pub duplicates: HashMap<String, Vec<PathBuf>>,
    pub scan_state: ScanState,
    pub thread_handle: Option<thread::JoinHandle<()>>,
    pub shared_results: Arc<Mutex<Option<(usize, HashMap<String, Vec<PathBuf>>)>>>,
    pub cancel_flag: Arc<AtomicBool>,
    pub live_count: Arc<Mutex<usize>>,
}

impl Default for DuplicateApp {
    fn default() -> Self {
        Self {
            scan_path: String::from("D:\\"),
            scan_status: "Idle".to_string(),
            file_count: 0,
            deleted_files: 0,
            deleted_bytes: 0,
            duplicates: HashMap::new(),
            scan_state: ScanState::Idle,
            thread_handle: None,
            shared_results: Arc::new(Mutex::new(None)),
            cancel_flag: Arc::new(AtomicBool::new(false)),
            live_count: Arc::new(Mutex::new(0)),
        }
    }
}

impl eframe::App for DuplicateApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Duplicate Cleaner");

            ui.horizontal(|ui| {
                ui.label("Folder to scan:");
                ui.text_edit_singleline(&mut self.scan_path);

                if self.scan_state == ScanState::Idle || self.scan_state == ScanState::Done || self.scan_state == ScanState::Cancelled {
                    if ui.button("Scan Now").clicked() {
                        self.deleted_files = 0;
                        self.deleted_bytes = 0;
                        self.duplicates.clear();
                        self.file_count = 0;
                        self.scan_status = "Scanning...".to_string();
                        self.scan_state = ScanState::Working;

                        let folder = PathBuf::from(self.scan_path.trim());
                        let shared = self.shared_results.clone();
                        let cancel_flag = self.cancel_flag.clone();
                        let live_count = self.live_count.clone();
                        self.cancel_flag.store(false, Ordering::Relaxed);
                        *self.live_count.lock().unwrap() = 0;

                        self.thread_handle = Some(thread::spawn(move || {
                            let files = scanner::scan_files_interruptible(
                                &[folder],
                                1_000_000,
                                cancel_flag.clone(),
                                live_count.clone(),
                            );

                            if cancel_flag.load(Ordering::Relaxed) {
                                return;
                            }

                            let file_count = files.len();
                            let dups = hashing::find_duplicates(&files);
                            let mut locked = shared.lock().unwrap();
                            *locked = Some((file_count, dups));
                        }));
                    }
                }

                if self.scan_state == ScanState::Working {
                    if ui.button("Cancel").clicked() {
                        self.cancel_flag.store(true, Ordering::Relaxed);
                        self.scan_status = "Cancelling...".to_string();
                    }
                }
            });

            if self.scan_state == ScanState::Working {
                if let Ok(mut locked) = self.shared_results.try_lock() {
                    if let Some((count, dups)) = locked.take() {
                        self.file_count = count;
                        self.duplicates = dups;
                        self.scan_status = format!(
                            "Done: {} duplicate groups from {} files",
                            self.duplicates.len(),
                            self.file_count
                        );
                        self.scan_state = ScanState::Done;
                    } else if self.cancel_flag.load(Ordering::Relaxed) {
                        self.scan_status = "Cancelled.".to_string();
                        self.scan_state = ScanState::Cancelled;
                    }
                }
            }

            if let Ok(count) = self.live_count.lock() {
                self.file_count = *count;
            }

            ui.separator();
            ui.label(format!("Status: {}", self.scan_status));
            ui.label(format!("Files scanned: {}", self.file_count));
            ui.label(format!("Duplicate groups found: {}", self.duplicates.len()));
            ui.label(format!("Files deleted: {}", self.deleted_files));
            ui.label(format!(
                "Total space reclaimed: {:.2} MB",
                self.deleted_bytes as f64 / (1024.0 * 1024.0)
            ));
            ui.separator();

            ui.heading("Duplicate Files Found:");
            let mut updated_duplicates = self.duplicates.clone();

            for (hash, files) in self.duplicates.iter() {
                if files.len() > 1 {
                    ui.collapsing(format!("{} ({} files)", hash, files.len()), |ui| {
                        for file in files {
                            ui.label(file.to_string_lossy());
                        }

                        if ui.button("🗑️ Delete All Except First").clicked() {
                            for file in files.iter().skip(1) {
                                if let Ok(meta) = std::fs::metadata(file) {
                                    self.deleted_bytes += meta.len();
                                }
                                if std::fs::remove_file(file).is_ok() {
                                    self.deleted_files += 1;
                                }
                            }
                            updated_duplicates.insert(hash.clone(), vec![files[0].clone()]);
                        }
                    });
                }
            }

            self.duplicates = updated_duplicates;
        });

        ctx.request_repaint();
    }
}
