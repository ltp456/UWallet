use std::process;
use eframe::emath::Align;
use egui::{Context, Direction};
use crate::app::IActivity;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingActivity {}


impl SettingActivity {
    pub fn new() -> Self {
        SettingActivity {}
    }

}

impl IActivity for SettingActivity{
    fn on_create(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Settings");
            });
            ui.separator();

            if ui.button("Exit").clicked(){
                process::exit(0);
            }


            ui.with_layout(egui::Layout::bottom_up(Align::Center), |ui| {
                ui.label("setting");
            })
        });
    }
}