use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::time::Duration;

use anyhow::Result;
use egui::Ui;
use log::debug;
use tokio::time;

use crate::executor::Executor;
use crate::navigation::ActivityKey;

use super::super::{IActivity, IView};
use super::super::AppState;

pub struct WelcomeActivity {
    ctx: egui::Context,
    navigate: Sender<ActivityKey>,
    executor: Arc<Executor>,
}

impl WelcomeActivity {
    pub fn new(ctx: egui::Context, navigate: Sender<ActivityKey>, executor: Arc<Executor>) -> WelcomeActivity {
        Self {
            ctx,
            navigate,
            executor,
        }
    }
}

impl IActivity for WelcomeActivity {
    fn on_create(&mut self, state: &AppState) {
        debug!("on_create");
        let ctx = self.ctx.clone();
        let navigate = self.navigate.clone();
        self.executor.spawn(async move {
            time::sleep(Duration::from_millis(2500)).await;
            debug!("navigate to password");
            navigate.send(ActivityKey::new("password")).unwrap();
            ctx.request_repaint();
        });
    }

    fn on_resume(&mut self, state: &AppState) {
        debug!("on_resume");
    }

    fn on_pause(&mut self, state: &AppState) {
        debug!("on_pause");
    }

    fn set_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, state: &AppState) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                ui.label(egui::RichText::new("UWallet").size(30 as f32).color(egui::Color32::WHITE));
            });
            ui.separator();
        });
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}