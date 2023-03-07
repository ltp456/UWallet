use std::sync::{Arc, Mutex};

use anyhow::Result;
use eframe::Frame;
use egui::Context;

use polkadot::{rpc::{*}};
use polkadot::rpc::client::Client;

use super::{widget::{*}};
use super::super::app::{IActivity,State};

#[derive(PartialEq)]
pub enum Chain {
    Ethereum,
    Polkadot,
}


pub struct TransferActivity {
    amount: String,
    dest: String,
    balance: String,
    submitted: bool,
    client: Arc<Client>,
    state:Arc<Mutex<State>>
}

impl TransferActivity {
    pub fn new(client: Arc<Client>,state:Arc<Mutex<State>>) -> Self {
        TransferActivity {
            client,
            state,
            amount: "".to_string(),
            dest: "".to_string(),
            balance: Default::default(),
            submitted: false,
        }
    }

    pub fn transfer(&self) -> Result<String> {
        println!("submitted: {} {}", self.amount, self.dest);
        Ok("".to_string())
    }
}


impl IActivity for TransferActivity {
    fn on_create(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let mut radio = Chain::Ethereum;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Transfer");
            });
            ui.separator();
            egui::ComboBox::from_label("select mainnet")
                .selected_text("polkadot")
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(&mut radio, Chain::Ethereum, "Ethereum");
                    ui.selectable_value(&mut radio, Chain::Polkadot, "Polkadot");
                });
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Balance: ");
                ui.label(&self.balance)
            });

            ui.horizontal(|ui| {
                ui.label("Amount: ");
                ui.add(egui::TextEdit::singleline(&mut self.amount).hint_text("input transfer amount value"));
            });

            ui.horizontal(|ui| {
                ui.label("Dest      :");
                ui.add(egui::TextEdit::singleline(&mut self.dest).hint_text("input transfer dest address"))
            });
            ui.separator();


            ui.vertical_centered(|ui| {
                if ui.button("Submit").clicked() {

                }
            })

        });
    }
}
