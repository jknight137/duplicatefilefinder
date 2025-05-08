use std::collections::HashMap;
use std::path::{PathBuf};
use eframe::egui::{self, CentralPanel, Context};
use crate::core::{hashing, scanner};

#[derive(PartialEq)]
enum ScanState {
    Idle,
    Scanning,
    Hashing,
    Complete,
}

pub struct DuplicateApp {
    pub scan_path: String,
    pub scan_state: ScanState,
    pub files_to_hash: Vec<PathBuf>,
    pub duplicates: HashMap<String, Vec<PathBuf>>,
    pub scan_status: String,
    pub file_count: usize,
    pub deleted_files: usize,
    pub deleted_bytes: u64,
    pub trigger_scan: bool,
}

impl Default for DuplicateApp {
    fn default() -> Self {
        Self {
            scan_path: String::from("C:\\"),
            scan_state: ScanState::Idle,
            files_to_hash: vec![],
            duplicates: HashMap::new(),
            scan_status: "Idle".to_string(),
            file_count: 0,
            deleted_files: 0,
            deleted_bytes: 0,
            trigger_scan: false,
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
                if ui.button("Scan Now").clicked() {
                    self.trigger_scan = true;
                }
            });

            // Handle scan trigger
            if self.trigger_scan {
                self.trigger_scan = false;
                self.scan_state = ScanState::Scanning;
                self.scan_status = "Scanning folder...".to_string();
                self.deleted_files = 0;
                self.deleted_bytes = 0;
                self.duplicates.clear();
                self.files_to_hash.clear();
                self.file_count = 0;

                let folder = PathBuf::from(self.scan_path.trim());
                self.files_to_hash = scanner::scan_files(&[folder], 1_000_000);
                self.file_count = self.files_to_hash.len();
                self.scan_state = ScanState::Hashing;
            }

            if self.scan_state == ScanState::Hashing {
                self.scan_status = format!("Hashing {} files...", self.file_count);
                self.duplicates = hashing::find_duplicates(&self.files_to_hash);
                self.scan_status = format!(
                    "Scan complete: {} duplicate groups",
                    self.duplicates.len()
                );
                self.scan_state = ScanState::Complete;
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

        ctx.request_repaint(); // Force GUI to update each frame
    }
}
