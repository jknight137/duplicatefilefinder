#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod providers;
mod gui;

use eframe::{run_native, NativeOptions};
use gui::app::DuplicateApp;

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_resizable(true),
        ..Default::default()
    };

    run_native(
        "Duplicate Cleaner",
        options,
        Box::new(|_cc| Ok(Box::new(DuplicateApp::default()))),
    )
}
