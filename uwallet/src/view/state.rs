use std::env::var;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use egui::Ui;
use log::debug;
use tokio::time;

use crate::executor::Executor;
use crate::IView;

use super::*;

pub fn error(ui: &mut Ui, msg: &str) {
    ui.label(egui::RichText::new(msg).size(15.0).color(egui::Color32::DARK_RED));
}

#[derive(Clone)]
pub struct DataModel {
    pub data_type: i32,
    pub data: Vec<u8>,
}


#[derive(Clone)]
pub enum ViewStatus {
    Loading,
    Fail(String),
    Success(DataModel),
    Normal,
}


pub fn hit_info(ui: &mut Ui, msg: &str) {
    ui.label(egui::RichText::new(msg).size(14.0).color(egui::Color32::GRAY));
}


pub struct BottomStatusBar {
    executor: Arc<Executor>,
    t_receiver: Receiver<i32>,
    t_sender: Sender<i32>,
    c_sender: Option<Sender<i32>>,

    ctx: egui::Context,
    change_loading_state: bool,
    inited_loading: bool,
    closed_loading: bool,
    show: bool,

}

impl BottomStatusBar {
    pub fn new(ctx: egui::Context, executor: Arc<Executor>) -> Self {
        let (t_sender, t_receiver) = channel::<i32>();
        Self {
            ctx,
            executor: executor.clone(),
            t_sender,
            t_receiver,
            c_sender: None,
            change_loading_state: false,
            inited_loading: false,
            closed_loading: false,
            show: true,
        }
    }

    pub fn start(&mut self) {
        self.show = true;
    }

    pub fn stop(&mut self) {
        // self.show = false;
        self.exit_loading();
    }


    pub fn loading_view(&mut self, ui: &mut Ui) {
        if !self.inited_loading {
            debug!("init loading view");
            self.inited_loading = !self.inited_loading;
            let (c_sender, c_receiver) = std::sync::mpsc::channel::<i32>();
            let sender = self.t_sender.clone();
            let ctx = self.ctx.clone();
            self.executor.spawn(async move {
                loop {
                    if let Ok(value) = c_receiver.try_recv() {
                        debug!("loading  view exit now");
                        break;
                    }
                    time::sleep(Duration::from_millis(600)).await;
                    sender.send(0).unwrap();
                    ctx.request_repaint();
                    debug!("send loading view sign");
                }
            });
            self.c_sender = Some(c_sender);
            self.closed_loading = false;
        }

        if let Ok(value) = self.t_receiver.try_recv() {
            self.change_loading_state = !self.change_loading_state
        }
        if self.change_loading_state {
            ui.label(egui::RichText::new("loading ..\t\t\t\t").size(13.0).color(egui::Color32::GRAY));
        } else {
            ui.label(egui::RichText::new("loading .. .. .. . ..\t").size(13.0).color(egui::Color32::GRAY));
        }
    }


    pub fn exit_loading(&mut self) {
        if !self.closed_loading {
            self.closed_loading = true;
            self.inited_loading = false;
            if let Some(sender) = &self.c_sender {
                sender.send(0).unwrap();
            }
        }
    }

    pub fn set_view(&mut self, ui: &mut Ui, status: &ViewStatus) {
        if !self.show {
            return;
        }
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            common::five_space(ui);
            common::center(ui, |ui| {
                common::twenty_space(ui);
                ui.vertical(|ui| {
                    match status {
                        ViewStatus::Loading => {
                            self.loading_view(ui);
                        }
                        ViewStatus::Fail(msg) => {
                            self.exit_loading();
                            ui.label(egui::RichText::new(format!("error:{}", msg)).size(13.0).color(egui::Color32::DARK_RED));
                            //ui.label(egui::RichText::new(format!("error:{}",msg)).size(13.0).color(egui::Color32::DARK_RED));
                        }
                        ViewStatus::Success(_) => {
                            self.exit_loading();
                            ui.label(egui::RichText::new("github: https://github.com/ltp456/UWallet").size(13.0).color(egui::Color32::DARK_GRAY));
                        }
                        ViewStatus::Normal => {
                            self.exit_loading();
                            ui.label(egui::RichText::new("github: https://github.com/ltp456/UWallet").size(13.0).color(egui::Color32::DARK_GRAY));
                        }
                    }
                });
            });
            common::ten_space(ui);
            ui.separator();
        });
    }
}


