use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use anyhow::Result;
use bip39::{Language, Mnemonic, MnemonicType};
use egui::Ui;
use log::debug;
use tokio::time;
use coreui::executor::Executor;
use coreui::lifecycle::ActName;
use coreui::state::AppState;

use polkadot::client::Client;



use crate::view::{common, state};
use crate::view::state::{BottomStatusBar, DataModel, ViewStatus};

use coreui::{IActivity, IView};


pub struct TransferActivity {
    ctx: egui::Context,
    navigate: Sender<ActName>,
    executor: Arc<Executor>,
    amount: String,
    dest_address: String,
    bottom_status_bar: BottomStatusBar,
    status_sender: Sender<ViewStatus>,
    status_receiver: Receiver<ViewStatus>,
    status: ViewStatus,
    client: Arc<Client>,
    tx_list: Vec<String>,

}

impl TransferActivity {
    pub fn new(ctx: egui::Context, navigate: Sender<ActName>, executor: Arc<Executor>, client: Arc<Client>) -> TransferActivity {
        let (status_sender, receiver) = std::sync::mpsc::channel::<ViewStatus>();
        Self {
            ctx: ctx.clone(),
            navigate,
            client,
            executor: executor.clone(),
            amount: "1234567891".to_string(),
            dest_address: "14dp76EwTctDZmX8bgJV3jC6KsnCCpjwzvjMpm4tc2AkJN2L".to_string(),
            status_sender,
            status_receiver: receiver,
            bottom_status_bar: BottomStatusBar::new(ctx, executor),
            status: ViewStatus::Normal,
            tx_list: vec![],
        }
    }

    pub fn transfer(&mut self, state: &AppState) {
        let mut from = String::new();
        let mut seed = String::new();
        if let Some(phrase) = state.get_value("PHRASE") {
            from = polkadot::keys::Key::address_from_phrase(&phrase, None);
            seed = format!("0x{}", polkadot::keys::Key::generate_seed(&phrase, None));
        }
        debug!("start transfer");
        self.status_sender.send(ViewStatus::Loading).unwrap();
        let ctx = self.ctx.clone();
        let sender = self.status_sender.clone();
        let client = self.client.clone();
        let address = self.dest_address.clone();
        let amount = self.amount.clone().parse::<u128>().unwrap();
        self.executor.spawn(async move {
            match client.transfer(seed, from, address, amount).await {
                Ok(result) => {
                    sender.send(ViewStatus::Success(DataModel { data_type: 0, data: result })).unwrap();
                }
                Err(e) => {
                    sender.send(ViewStatus::Fail(e.to_string())).unwrap();
                }
            }
            ctx.request_repaint();
        });
    }


    pub fn navigate(&mut self, key: ActName) {
        self.navigate.send(key).unwrap();
        self.ctx.request_repaint();
    }
}

impl IActivity for TransferActivity {
    fn on_create(&mut self,ctx: &egui::Context, state: &AppState) {
        debug!("on_create");
    }

    fn on_resume(&mut self,ctx: &egui::Context, state: &AppState) {
        debug!("on_resume");
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
        if let Ok(mut data) = self.status_receiver.try_recv() {
            self.status = data.clone();
            match &data {
                ViewStatus::Success(data) => {
                    self.tx_list.push(String::from_utf8(data.data.clone()).unwrap())
                }
                _ => {}
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            common::five_space(ui);
            common::title(ui, "Transfer");
            common::five_space(ui);
            ui.separator();
            common::five_space(ui);
            common::simple_input_label(ui, "\t\tDestAddress\t", "input address", &mut self.dest_address);
            common::ten_space(ui);
            common::simple_input_label(ui, "\t\tAmount \t\t\t", "input amount", &mut self.amount);
            common::fifteen_space(ui);
            ui.separator();
            common::thirty_space(ui);

            if self.tx_list.len() > 0 {
                common::label(ui, "Transaction history");
                ui.separator();
            }
            for item in &self.tx_list {
                common::hyperlink_to(ui, &format!("Hash: {}",item));
                common::five_space(ui);
            }
            if common::right_bottom_button(ui, "Submit") {
                self.transfer(state);
            }
            self.bottom_status_bar.set_view(ui, &self.status);
        });
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}