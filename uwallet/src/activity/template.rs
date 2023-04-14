use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::time::Duration;

use anyhow::Result;
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
use crate::view::{common, state};



pub struct TemplateActivity {

    phrase: String,
}

impl TemplateActivity {
    pub fn new() -> TemplateActivity {
        Self {
            phrase: Default::default(),
        }
    }
    pub fn generate_phrase(&mut self) {}

    pub fn confirm_phrase(&mut self) {

    }
}

impl IActivity for TemplateActivity {
    fn on_create(&mut self,ctx: &egui::Context, state: &AppState) {
        debug!("on_create");
    }

    fn on_resume(&mut self,ctx: &egui::Context, state: &AppState) {
        debug!("on_resume");
    }

    fn on_pause(&mut self,ctx: &egui::Context, state: &AppState) {
        debug!("on_pause");
    }

    fn set_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, state: &AppState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("template");
        });
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}