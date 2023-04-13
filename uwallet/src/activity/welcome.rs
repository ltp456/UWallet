use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::time::Duration;

use anyhow::Result;
use egui::Ui;
use log::debug;
use tokio::time;
use coreui::executor::Executor;
use coreui::lifecycle::ActName;
use coreui::state::AppState;

use crate::{IActivity, IView};

pub struct WelcomeActivity {
    ctx: egui::Context,
    navigate: Sender<ActName>,
    executor: Arc<Executor>,
}

impl WelcomeActivity {
    pub fn new(ctx: egui::Context, navigate: Sender<ActName>, executor: Arc<Executor>) -> WelcomeActivity {
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
            navigate.send(ActName::new("password")).unwrap();
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