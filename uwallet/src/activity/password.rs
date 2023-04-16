use std::sync::mpsc::Sender;

use anyhow::{anyhow, Result};
use log::debug;

use coreui::{
    eframe,
    egui,
    executor::{Executor, EXECUTOR},
    IActivity,
    IView,
    lifecycle::{ActName, start_activity},
    state::AppState,
};

use crate::{
    view::{common, state},
};
use crate::constants::{*};

pub struct PasswordActivity {
    password: String,
    confirm_pwd: String,
    pwd_error: bool,
    new_password: bool,

}

impl PasswordActivity {
    pub fn new() -> PasswordActivity {
        Self {
            password: "abcd".to_string(),
            confirm_pwd: "abcd".to_string(),
            pwd_error: false,
            new_password: true,
        }
    }

    fn check_password(&self, state: &AppState) -> Result<String> {
        let data = state.get_encode_data().unwrap();
        if let Ok(decode_data) = utils::aes::simple_decode(data.as_bytes(), self.password.as_bytes()) {
            state.init_data(&decode_data);
            return Ok("ok".to_owned());
        } else {
            return Err(anyhow!(""));
        }
    }


    fn set_new_password(&self, state: &AppState) -> Result<String> {
        if self.password != self.confirm_pwd || self.password == "" {
            return Err(anyhow!("password not match"));
        }
        state.set_value(PWD_KEY, &self.password);
        return Ok("Ok".to_string());
    }

    fn navigate_activity(&self, state: &AppState) {
        if state.exists(PHRASE_KEY) {
            start_activity(ActName::new(HOME)).unwrap();
        } else {
            start_activity(ActName::new(PHRASE)).unwrap();
        }
    }

    fn set_new_password_view(&mut self, ctx: &egui::Context, state: &AppState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            common::ten_space(ui);
            common::title(ui, "UWallet");
            common::ten_space(ui);
            ui.separator();
            common::thirty_space(ui);
            common::center(ui, |ui| {
                ui.vertical_centered(|ui| {
                    common::single_input_label(ui, "Password\t\t\t\t", "input password", &mut self.password);
                    common::thirty_space(ui);
                    common::single_input_label(ui, "ConfirmPassword", "input password", &mut self.confirm_pwd);
                    common::thirty_space(ui);
                });
            });
            common::ten_space(ui);
            if self.pwd_error {
                ui.vertical_centered(|ui| {
                    common::five_space(ui);
                    state::error(ui, "hint: password not match");
                });
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                common::fifteen_space(ui);
                if common::button(ui, "Submit").clicked() {
                    if let Ok(ok) = self.set_new_password(state) {
                        return self.navigate_activity(state);
                    } else {
                        self.pwd_error = true;
                    }
                }
            });
            common::fifteen_space(ui);
        });
    }

    fn input_password_view(&mut self, ctx: &egui::Context, state: &AppState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            common::ten_space(ui);
            common::title(ui, "UWallet");
            common::ten_space(ui);
            ui.separator();
            common::thirty_space(ui);
            common::center(ui, |ui| {
                ui.vertical_centered(|ui| {
                    common::thirty_space(ui);
                    common::thirty_space(ui);
                    common::single_input_label(ui, "Password\t", "input password", &mut self.password);
                    common::fifteen_space(ui);
                });
            });
            common::ten_space(ui);
            if self.pwd_error {
                ui.vertical_centered(|ui| {
                    common::five_space(ui);
                    state::error(ui, "hint: password not match");
                });
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                common::fifteen_space(ui);
                if common::button(ui, "Submit").clicked() {
                    if let Ok(ok) = self.check_password(state) {
                        self.navigate_activity(state);
                    } else {
                        self.pwd_error = true;
                    }
                }
            });
            common::fifteen_space(ui);
        });
    }
}


impl IActivity for PasswordActivity {
    fn on_create(&mut self, ctx: &egui::Context, state: &AppState) {
        if state.encode_data_exists() {
            self.new_password = false;
        } else {
            self.new_password = true;
        }
        debug!("on_create");
    }

    fn on_resume(&mut self, ctx: &egui::Context, state: &AppState) {
        debug!("on_resume");
    }

    fn on_pause(&mut self, ctx: &egui::Context, state: &AppState) {
        debug!("on_pause");
    }

    fn set_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, state: &AppState) {
        if self.new_password {
            self.set_new_password_view(ctx, state);
        } else {
            self.input_password_view(ctx, state)
        }
    }
}

