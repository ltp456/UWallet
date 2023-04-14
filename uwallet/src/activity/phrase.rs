use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::time::Duration;

use anyhow::Result;
use bip39::{Language, Mnemonic, MnemonicType};
use egui::Ui;
use log::debug;
use tokio::time;
use coreui::executor::Executor;
use coreui::lifecycle::ActName;

use crate::view::{common, state};

use super::super::{IActivity, IView};
use super::super::AppState;

pub struct PhraseActivity {
    ctx: egui::Context,
    navigate: Sender<ActName>,
    executor: Arc<Executor>,
    phrase: String,
    import_ui: bool,
}

impl PhraseActivity {
    pub fn new(ctx: egui::Context, navigate: Sender<ActName>, executor: Arc<Executor>) -> PhraseActivity {
        Self {
            ctx,
            navigate,
            executor,
            phrase: Default::default(),
            import_ui: false,
        }
    }
    pub fn generate_phrase(&mut self) {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        self.phrase = mnemonic.phrase().to_string();
        self.ctx.request_repaint();
    }

    pub fn confirm_phrase(&mut self, state: &AppState) {
        if self.phrase == "" {
            return;
        }
        state.set_value("PHRASE".to_owned(), self.phrase.clone());
        self.navigate.send(ActName::new("home")).unwrap();
        self.ctx.request_repaint();
    }
}

impl IActivity for PhraseActivity {
    fn on_create(&mut self,ctx: &egui::Context, state: &AppState) {
        debug!("on_create");
    }

    fn on_resume(&mut self,ctx: &egui::Context, state: &AppState) {
        debug!("on_resume");
    }

    fn on_pause(&mut self, ctx: &egui::Context,state: &AppState) {
        debug!("on_pause");
    }

    fn set_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, state: &AppState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            common::ten_space(ui);
            common::title(ui, "UWallet");
            common::ten_space(ui);
            ui.separator();
            common::thirty_space(ui);

            if self.import_ui {
                common::center(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            if common::button(ui, "Import").clicked() {}
                            if common::ssmall_button(ui, "generate new phrase?").clicked() {
                                self.import_ui = false;
                                self.phrase = "".to_string();
                            }
                        });
                        common::thirty_space(ui);
                        common::simple_input_label(ui, "Phrase:", "please input your phrase", &mut self.phrase);
                    });
                });
            } else {
                common::center(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        common::center(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                if common::button(ui, "Generate").clicked() {
                                    self.generate_phrase();
                                }
                                if common::ssmall_button(ui, "exists phrase,import?").clicked() {
                                    self.import_ui = true;
                                    self.phrase = "".to_string();
                                }
                            });
                        });
                        common::thirty_space(ui);
                        common::label(ui, &self.phrase);
                    });
                });
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                common::fifteen_space(ui);
                if self.phrase != "" {
                    common::ten_space(ui);
                    if common::button(ui, "Submit").clicked() {
                        self.confirm_phrase(state);
                    }
                }
            });
        });
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}