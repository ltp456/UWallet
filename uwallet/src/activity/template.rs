use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::time::Duration;

use anyhow::Result;
use egui::Ui;
use log::debug;
use tokio::time;

use crate::executor::Executor;
use crate::navigation::ActivityKey;
use crate::view::{common, state};

use super::super::{IActivity, IView};
use super::super::AppState;


pub struct TemplateActivity {
    ctx: egui::Context,
    navigate: Sender<ActivityKey>,
    executor: Arc<Executor>,
    phrase: String,
}

impl TemplateActivity {
    pub fn new(ctx: egui::Context, navigate: Sender<ActivityKey>, executor: Arc<Executor>) -> TemplateActivity {
        Self {
            ctx,
            navigate,
            executor,
            phrase: Default::default(),
        }
    }
    pub fn generate_phrase(&mut self) {}

    pub fn confirm_phrase(&mut self) {
        self.navigate.send(ActivityKey::new("phrase")).unwrap();
        self.ctx.request_repaint();
    }
}

impl IActivity for TemplateActivity {
    fn on_create(&mut self,state: &AppState) {
        debug!("on_create");
    }

    fn on_resume(&mut self,state: &AppState) {
        debug!("on_resume");
    }

    fn on_pause(&mut self,state: &AppState) {
        debug!("on_pause");
    }

    fn set_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame,state: &AppState) {
        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}