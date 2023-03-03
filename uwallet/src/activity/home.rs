use super::{interface::{*}};

pub struct HomeActivity {}

impl HomeActivity {
    pub(crate) fn on_create(ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical_centered(|ui|{
                ui.heading("Home");
            });
            ui.add_space(20 as f32);
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Address:");
                ui.label("sdfsdfsdfsfsdfsdfsdf")
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Balance:");
                ui.label("1000000 DOT")
            })
        });
    }
}
