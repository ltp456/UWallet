
use ext::keys::{*, Key};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WelcomeActivity {
    complete: bool,
    phrase: String,
    confirm: bool,
}

impl WelcomeActivity {
    pub fn new() -> Self {
        WelcomeActivity {
            complete: false,
            confirm: false,
            phrase: Default::default(),
        }
    }

    pub fn on_create(&mut self, ctx: &egui::Context, _frame: &eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("UWallet");
            });

            ui.add_space(30 as f32);
            ui.separator();
            ui.add_space(3 as f32);
            if ui.button("generate mnemonic").clicked() && !self.complete {
                self.phrase = Key::generate_phrase(MnemonicType::Words12);
                self.complete = true;
            }
            if self.complete {
                ui.label(self.phrase.clone());
            }

            if ui.button("Confirm").clicked() {
                self.confirm = true
            }
        });
    }

    pub fn get_status(&mut self) -> (bool, String) {
        (self.confirm, self.phrase.clone())
    }
}