use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::time::Duration;

use anyhow::Result;
use log::debug;
use tokio::time;

use coreui::{
    eframe,
    egui,
    executor::{Executor, EXECUTOR},
    IActivity,
    IView,
    lifecycle::{ActName, start_activity},
    state::AppState,
};
use crate::PASSWORD;

pub struct WelcomeActivity {}

impl WelcomeActivity {
    pub fn new() -> WelcomeActivity {
        Self {}
    }
}

impl IActivity for WelcomeActivity {
    fn on_create(&mut self, ctx: &egui::Context, state: &AppState) {
        debug!("on_create");
        EXECUTOR.spawn(async move {
            time::sleep(Duration::from_millis(2500)).await;
            debug!("navigate to password");
            start_activity(ActName::new(PASSWORD)).unwrap();
        });
    }

    fn on_resume(&mut self, ctx: &egui::Context, state: &AppState) {
        debug!("on_resume");
    }

    fn on_pause(&mut self, ctx: &egui::Context, state: &AppState) {
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