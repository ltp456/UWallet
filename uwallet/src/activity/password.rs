use std::sync::mpsc::Sender;

use anyhow::{anyhow, Result};
use egui::Ui;
use log::debug;
use coreui::lifecycle::ActName;
use coreui::state::AppState;

use crate::{view};

use coreui::{IActivity, IView};
use super::super::view::{*};

pub struct PasswordActivity {
    password: String,
    confirm_pwd: String,
    navigate: Sender<ActName>,
    pwd_error: bool,
    new_password: bool,
    ctx: egui::Context,
}

impl PasswordActivity {
    pub fn new(ctx: egui::Context, navigate: Sender<ActName>) -> PasswordActivity {
        Self {
            ctx,
            navigate,
            password: "abcd".to_string(),
            confirm_pwd: "abcd".to_string(),
            pwd_error: false,
            new_password: true,
        }
    }

    fn check_password(&self, state: &AppState) -> Result<String> {
        if let Some(password) = state.get_value("PWD") {
            if self.password == password {
                return Ok("Ok".to_string());
            }
        }
        return Err(anyhow!("password not match"));
    }

    fn set_new_password(&self, state: &AppState) -> Result<String> {
        if self.password != self.confirm_pwd || self.password == "" {
            return Err(anyhow!("password not match"));
        }
        state.set_value("PWD".to_owned(), self.password.clone());
        return Ok("Ok".to_string());
    }

    fn navigate_phrase(&self, state: &AppState) {
        if state.exists("PHRASE") {
            self.navigate.send(ActName::new("home")).unwrap();
            //self.navigate.send(ActivityKey::new("phrase")).unwrap();
        } else {
            self.navigate.send(ActName::new("phrase")).unwrap();
        }
        self.ctx.request_repaint();
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
                        return self.navigate_phrase(state);
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
                        return self.navigate_phrase(state);
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
    fn on_create(&mut self,ctx: &egui::Context, state: &AppState) {
        if state.exists("PWD") {
            self.new_password = false;
        } else {
            self.new_password = true;
        }
        debug!("on_create");
    }

    fn on_resume(&mut self,ctx: &egui::Context, state: &AppState) {
        debug!("on_resume");
    }

    fn on_pause(&mut self, ctx: &egui::Context,state: &AppState) {
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

