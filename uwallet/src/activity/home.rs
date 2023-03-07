use super::{interface::{*}};
#[derive(serde::Deserialize, serde::Serialize)]
pub struct HomeActivity {
    address: String,
}

impl HomeActivity {
    pub fn new() -> Self {
        HomeActivity {
            address: Default::default(),
        }
    }

    pub fn set(&mut self, address: String) {
        self.address = address
    }

    pub fn on_create(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Home");
            });
            ui.add_space(20 as f32);
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Address:");
                ui.label(&self.address)
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Balance:");
                ui.label("1000000 DOT")
            })
        });
    }
}
