use std::collections::HashMap;
use std::path::PathBuf;

use eframe::egui::{self, CentralPanel, Context};
use crate::core::{hashing, scanner};

pub struct DuplicateApp {
    pub scan_path: String,
    pub duplicates: HashMap<String, Vec<PathBuf>>,
    pub scan_status: String,
    pub file_count: usize,
}

impl Default for DuplicateApp {
    fn default() -> Self {
        Self {
            scan_path: String::from("C:\\"), // Default Windows path
            duplicates: HashMap::new(),
            scan_status: "Idle".to_string(),
            file_count: 0,
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
                    let folder = PathBuf::from(self.scan_path.trim());

                    self.scan_status = format!("Scanning {}...", folder.display());
                    self.duplicates.clear();
                    self.file_count = 0;

                    let files = scanner::scan_files(&[folder], 1_000_000);
                    self.file_count = files.len();

                    self.scan_status = format!("Hashing {} files...", self.file_count);
                    self.duplicates = hashing::find_duplicates(&files);

                    self.scan_status = format!(
                        "Scan complete: {} duplicate groups",
                        self.duplicates.len()
                    );
                }
            });

            ui.separator();
            ui.label(format!("Status: {}", self.scan_status));
            ui.label(format!("Files scanned: {}", self.file_count));
            ui.label(format!("Duplicate groups found: {}", self.duplicates.len()));
            ui.separator();

            ui.heading("Duplicate Files Found:");
            for (hash, files) in &self.duplicates {
                if files.len() > 1 {
                    ui.collapsing(format!("{} ({} files)", hash, files.len()), |ui| {
                        for file in files {
                            ui.label(file.to_string_lossy());
                        }
                    });
                }
            }
        });
    }
}
