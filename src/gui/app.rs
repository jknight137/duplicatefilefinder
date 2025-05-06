use eframe::egui::{self, Context};

pub struct DuplicateApp {
    // add state fields as needed
}

impl Default for DuplicateApp {
    fn default() -> Self {
        Self { }
    }
}

impl eframe::App for DuplicateApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Duplicate Cleaner");
            if ui.button("Scan duplicates").clicked() {
                // TODO: call core scanning logic
            }
        });
    }
}
