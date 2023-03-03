use eframe::Frame;
use egui::Context;

#[derive(PartialEq)]
pub enum Chain {
    Ethereum,
    Polkadot,
}

pub struct TransferActivity {}

impl TransferActivity {
   pub  fn on_create(ctx: &Context,_frame: &eframe::Frame) {
       let mut radio = Chain::Ethereum;
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("select mainnet")
                .selected_text("polkadot")
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value( &mut radio, Chain::Ethereum, "Ethereum");
                    ui.selectable_value( &mut radio, Chain::Polkadot, "Polkadot");
                });
        });
    }
}