use std::sync::{Arc, Mutex};

use egui::Context;

use super::super::app::{IActivity, Page, State};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SplashActivity {
    input: String,
    submit: bool,
    state: Arc<Mutex<State>>,
}


impl SplashActivity {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        SplashActivity {
            input: "abcd".to_string(),
            submit: false,
            state,
        }
    }
}

impl IActivity for SplashActivity {
    fn on_create(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("UWallet");
            });

            ui.add_space(30 as f32);
            ui.separator();
            ui.add_space(3 as f32);
            ui.vertical_centered(|ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Password: ");
                    ui.add(egui::TextEdit::singleline(&mut self.input).hint_text("input your password"));
                });
            });
            ui.add_space(3 as f32);

            if ui.button(egui::RichText::new("Submit").size(20 as f32).color(egui::Color32::WHITE)).clicked() {
                println!("clicked");
                self.state.lock().unwrap().current_page = Page::Welcome;
            }
        });
    }
}