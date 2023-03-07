use std::sync::{Arc, Mutex};
use egui::Context;
use polkadot::keys::{*, Key};
use super::super::app::{IActivity,Page,State};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WelcomeActivity {
    complete: bool,
    phrase: String,
    confirm: bool,
    state: Arc<Mutex<State>>,
}

impl WelcomeActivity {
    pub fn new(state: Arc<Mutex<State>>,) -> Self {
        WelcomeActivity {
            state,
            complete: false,
            confirm: false,
            phrase: Default::default(),
        }
    }

}


impl IActivity for WelcomeActivity {
    fn on_create(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
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
               self.state.lock().unwrap().current_page = Page::Home;
            }
        });
    }
}


