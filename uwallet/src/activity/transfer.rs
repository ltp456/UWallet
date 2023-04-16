use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use anyhow::Result;
use bip39::{Language, Mnemonic, MnemonicType};
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
use polkadot::client::Client;

use crate::{
    constants::{*},
    view::{common, state::{BottomStatusBar, DataModel, ViewStatus}},
};

pub struct TransferActivity {
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
    pub fn new(ctx: egui::Context, client: Arc<Client>) -> TransferActivity {
        let (status_sender, receiver) = std::sync::mpsc::channel::<ViewStatus>();
        Self {
            client,
            amount: "".to_string(),
            dest_address: "".to_string(),
            status_sender,
            status_receiver: receiver,
            bottom_status_bar: BottomStatusBar::new(ctx),
            status: ViewStatus::Normal,
            tx_list: vec![],
        }
    }

    pub fn transfer(&mut self, ctx: &egui::Context, state: &AppState) {
        if self.amount == "" || self.dest_address == "" {
            return;
        }

        let mut from = String::new();
        let mut seed = String::new();
        if let Some(phrase) = state.get_value(PHRASE_KEY) {
            from = polkadot::keys::Key::address_from_phrase(&phrase, None);
            seed = format!("0x{}", polkadot::keys::Key::generate_seed(&phrase, None));
        }
        debug!("start transfer");
        self.status_sender.send(ViewStatus::Loading).unwrap();
        let ctx = ctx.clone();
        let sender = self.status_sender.clone();
        let client = self.client.clone();
        let address = self.dest_address.clone();
        let amount = self.amount.clone().parse::<u128>().unwrap();
        EXECUTOR.spawn(async move {
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
        start_activity(key).unwrap();
    }
}

impl IActivity for TransferActivity {
    fn on_create(&mut self, ctx: &egui::Context, state: &AppState) {
        debug!("on_create");
    }

    fn on_resume(&mut self, ctx: &egui::Context, state: &AppState) {
        debug!("on_resume");
    }

    fn on_pause(&mut self, ctx: &egui::Context, state: &AppState) {
        debug!("on_pause");
        self.bottom_status_bar.stop();
    }

    fn set_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, state: &AppState) {
        let (home, transfer, setting) = common::left_menu(ctx);
        if home {
            self.navigate(ActName::new(HOME));
        } else if transfer {
            self.navigate(ActName::new(TRANSFER));
        } else if setting {
            self.navigate(ActName::new(SETTING));
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
                common::hyperlink_to(ui, &format!("Hash: {}", item));
                common::five_space(ui);
            }
            if common::right_bottom_button(ui, "Submit") {
                self.transfer(ctx, state);
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