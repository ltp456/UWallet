use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use anyhow::Result;
use bip39::{Language, Mnemonic, MnemonicType};
use codec::{Decode, Encode};
use log::{debug, error};
use tokio::time;

use coreui::{
    eframe,
    egui,
    executor::{Executor, EXECUTOR},
    IActivity,
    IView,
    lifecycle::ActName,
    state::AppState,
};
use coreui::lifecycle::start_act;
use polkadot::{
    client::Client,
    rpc::{*},
    rpc::types::AccountInfo,
};

use crate::view::{common, state::{BottomStatusBar, DataModel, ViewStatus}};

pub struct HomeActivity {
    balance: String,
    nonce: String,
    address: String,

    bottom_status_bar: BottomStatusBar,
    view_status_sender: Sender<ViewStatus>,
    view_status_receiver: Receiver<ViewStatus>,
    client: Arc<Client>,
    status: ViewStatus,
}

impl HomeActivity {
    pub fn new(ctx: egui::Context, client: Arc<Client>) -> HomeActivity {
        let (sender, receiver) = std::sync::mpsc::channel::<ViewStatus>();
        Self {
            balance: "0.0".to_string(),
            address: "15QFBQY6TF6Abr6vA1r6opRh6RbRSMWgBC1PcCMDDzRSEXf5".to_string(),
            nonce: "0".to_string(),
            bottom_status_bar: BottomStatusBar::new(ctx.clone()),
            view_status_sender: sender,
            view_status_receiver: receiver,
            client,
            status: ViewStatus::Loading,
        }
    }

    pub fn navigate(&mut self, key: ActName) {
        start_act(key).unwrap();
    }
}

impl IActivity for HomeActivity {
    fn on_create(&mut self, ctx: &egui::Context, state: &AppState) {
        debug!("on_create");
        if let Some(phrase) = state.get_value("PHRASE") {
            self.address = polkadot::keys::Key::address_from_phrase(&phrase, None);
            debug!("address: {}",self.address);
        }
    }

    fn on_resume(&mut self, ctx: &egui::Context, state: &AppState) {
        debug!("on_resume");
        self.view_status_sender.send(ViewStatus::Loading).unwrap();
        let sender = self.view_status_sender.clone();
        let ctx = ctx.clone();
        let client = self.client.clone();
        let address = self.address.clone();
        EXECUTOR.spawn(async move {
            debug!("start request account info");
            match client.system_account(&address).await {
                Ok(account) => {
                    debug!("request account ");
                    sender.send(ViewStatus::Success(DataModel { data_type: 0, data: account })).unwrap();
                }
                Err(e) => {
                    error!("request account error");
                    sender.send(ViewStatus::Fail(e.to_string())).unwrap();
                }
            }
            ctx.request_repaint();
        });
    }

    fn on_pause(&mut self, ctx: &egui::Context, state: &AppState) {
        debug!("on_pause");
        self.bottom_status_bar.stop();
    }

    fn set_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, state: &AppState) {

        // left menu
        let (home, transfer, setting) = common::left_menu(ctx);
        if home {
            self.navigate(ActName::new("home"));
        } else if transfer {
            self.navigate(ActName::new("transfer"));
        } else if setting {
            self.navigate(ActName::new("setting"));
        }
        //

        egui::CentralPanel::default().show(ctx, |ui| {
            common::five_space(ui);
            common::title(ui, "Polkadot");
            common::five_space(ui);
            ui.separator();
            common::five_space(ui);
            if let Ok(mut data) = self.view_status_receiver.try_recv() {
                self.status = data.clone();
                match &data {
                    ViewStatus::Success(data) => {
                        let account: AccountInfo = Decode::decode(&mut data.data.as_slice()).unwrap();
                        self.balance = format!("{}", account.data.free);
                        self.nonce = format!("{}", account.nonce);
                    }
                    _ => {}
                }
            }
            common::single_label(ui, "Address:\t\t", &self.address);
            common::ten_space(ui);
            common::single_label(ui, "Balance:\t\t", &self.balance);
            common::five_space(ui);
            common::single_label(ui, "Nonce\t:\t\t", &self.nonce);
            common::five_space(ui);
            ui.separator();
            self.bottom_status_bar.set_view(ui, &self.status);
        });
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}