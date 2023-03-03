use eframe::emath::Align;
use egui::Direction;

pub struct SettingActivity {}

impl SettingActivity {
    pub fn on_create(ctx: &egui::Context, _frame: &eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Settings");
            });
            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(Align::Center), |ui| {
                ui.label("setting");
            })
        });
    }
}