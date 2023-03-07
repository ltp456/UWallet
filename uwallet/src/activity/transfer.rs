use eframe::Frame;
use egui::Context;

use super::{widget::{*}};

#[derive(PartialEq)]
pub enum Chain {
    Ethereum,
    Polkadot,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TransferActivity {}

impl TransferActivity {
    pub fn on_create(ctx: &Context, _frame: &eframe::Frame) {
        let mut radio = Chain::Ethereum;
        let mut input = String::new();
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical_centered(|ui|{
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
                ui.label("100000 DOT")
            });

            ui.horizontal(|ui| {
                ui.label("Amount: ");
                ui.add(egui::TextEdit::singleline(&mut input).hint_text("input transfer amount value"));
            });

            ui.horizontal(|ui| {
                ui.label("Dest      :");
                ui.add(egui::TextEdit::singleline(&mut input).hint_text("input transfer dest address"))
            });
            ui.separator();

            ui.vertical_centered(|ui| {
                if ui.button("Submit").clicked() {

                }
            })
        });
    }
}