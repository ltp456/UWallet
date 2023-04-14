use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use anyhow::Result;
use bip39::{Language, Mnemonic, MnemonicType};

use log::debug;
use tokio::time;

use coreui::{
    executor::{Executor, EXECUTOR},
    lifecycle::ActName,
    state::AppState,
    IActivity,
    egui,
    IView,
    eframe,
};
use coreui::lifecycle::start_act;

use crate::view::{common, state};
use crate::view::state::{BottomStatusBar, ViewStatus};



pub struct SettingActivity {
    balance: String,
    endpoint: String,
    bottom_status_bar: BottomStatusBar,
    status_sender: Sender<ViewStatus>,
    status_receiver: Receiver<ViewStatus>,
    status: ViewStatus,
    hit_info: String,

}

impl SettingActivity {
    pub fn new(ctx:egui::Context) -> SettingActivity {
        let (sender, receiver) = std::sync::mpsc::channel::<ViewStatus>();
        Self {
            balance: "11231231231231231".to_string(),
            endpoint: "".to_string(),
            status_sender: sender,
            status_receiver: receiver,
            bottom_status_bar: BottomStatusBar::new(ctx),
            status: ViewStatus::Normal,
            hit_info: "".to_string(),
        }
    }

    pub fn navigate(&mut self, key: ActName) {
      start_act(key).unwrap();
    }
}

impl IActivity for SettingActivity {
    fn on_create(&mut self,ctx: &egui::Context, state: &AppState) {
        debug!("on_create");
    }

    fn on_resume(&mut self,ctx: &egui::Context, state: &AppState) {
        debug!("on_resume");
        self.hit_info = "".to_string();
    }

    fn on_pause(&mut self,ctx: &egui::Context, state: &AppState) {
        debug!("on_pause");
        self.bottom_status_bar.stop();
    }

    fn set_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, state: &AppState) {
        let (home, transfer, setting) = common::left_menu(ctx);
        if home {
            self.navigate(ActName::new("home"));
        } else if transfer {
            self.navigate(ActName::new("transfer"));
        } else if setting {
            self.navigate(ActName::new("setting"));
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Ok(mut data) = self.status_receiver.try_recv() {
                self.status = data.clone();
                match &data {
                    ViewStatus::Success(data) => {}
                    _ => {}
                }
            }

            common::five_space(ui);
            common::title(ui, "Setting");
            common::five_space(ui);
            ui.separator();

            common::center(ui, |ui| {
                ui.vertical_centered(|ui| {
                    common::fifteen_space(ui);
                    if common::small_button(ui, "\t\t\t\t\t\t\t\t\tBakUp\t\t\t\t\t\t\t\t\t").clicked() {
                        if let Some(phrase) = state.get_value("PHRASE") {
                            utils::copy_to_clipboard(&phrase).unwrap();
                            self.hit_info = "Info: ** Phrase Copied to clipboard **".to_string();
                        }
                    };
                    //common::simple_input_label(ui, "\t\tAdd RPC Endpoint:\t", "input endpoint", &mut self.endpoint);
                    common::fifteen_space(ui);

                    // ui.separator();
                    // common::fifteen_space(ui);
                    // common::small_button(ui, "\t\t\t\t\t\t\t\t\tSetRPC\t\t\t\t\t\t\t\t\t");
                    // common::fifteen_space(ui);
                    //ui.separator();

                    common::fifteen_space(ui);
                    if common::small_button(ui, "\t\t\t\t\t\t\t\t\t\tExit\t\t\t\t\t\t\t\t\t\t").clicked() {
                        debug!("exit application now");
                        _frame.close();
                    };
                    common::fifteen_space(ui);
                    ui.separator();
                    common::fifteen_space(ui);
                    if self.hit_info != "" {
                        state::hit_info(ui, &self.hit_info);
                    }
                });
            });


            // if common::right_bottom_button(ui, "Submit") {}

            self.bottom_status_bar.set_view(ui, &self.status);
        });
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}