use std::sync::{Arc, Mutex};

use egui::Context;
use log::info;

use polkadot::rpc::client::Client;

use super::super::{
    app::{IActivity, State},
    executor::Executor,
};

#[derive(Clone)]
struct HomeData {
    pub address: String,
    pub balance: String,
    pub is_refreshing: bool,
}

impl Default for HomeData {
    fn default() -> Self {
        Self {
            address: Default::default(),
            balance: Default::default(),
            is_refreshing: false,
        }
    }
}


pub struct HomeActivity {
    data: Arc<Mutex<HomeData>>,
    client: Arc<Client>,
    state: Arc<Mutex<State>>,
    executor: Executor,
}

impl HomeActivity {
    pub fn new(client: Arc<Client>, state: Arc<Mutex<State>>) -> Self {
        HomeActivity {
            data: Arc::new(Mutex::new(HomeData::default())),
            executor: Executor::new(),
            client,
            state,
        }
    }

    pub fn update(&self, ctx: Context) {
        let client = self.client.clone();
        if self.data.lock().unwrap().is_refreshing {
            return;
        }
        self.data.lock().unwrap().is_refreshing = true;
        let data = self.data.clone();
        self.executor.spawn(async move {
            info!("ddd");
            let address = "16mBaA4BPtJzxLchgbHkimRamd4PjnEpELn2N1TS86Hv3NJ7".to_string();
            let account_info = client.system_account(address.clone()).unwrap();
            data.lock().unwrap().address = address;
            data.lock().unwrap().balance = format!("{}", account_info.data.free);
            data.lock().unwrap().is_refreshing = false;
            ctx.request_repaint();
        });
    }

    pub fn on_create(&self, ctx: &Context, _frame: &mut eframe::Frame) {
        let data = self.data.clone().lock().unwrap().clone();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Home");
            });
            ui.add_space(20 as f32);
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Address:");
                ui.label(data.address)
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Balance:");
                ui.label(data.balance)
            })
        });
    }
}

