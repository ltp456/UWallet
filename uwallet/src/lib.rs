use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

pub use app::WalletApp;
use coreui::state::AppState;

mod app;
mod activity;
mod view;


pub trait IView {
    fn view(&mut self, ui: &mut egui::Ui);
}

pub trait IActivity {
    fn on_create(&mut self, ctx: &egui::Context, state: &AppState);

    fn on_resume(&mut self, ctx: &egui::Context, state: &AppState);

    fn on_pause(&mut self, ctx: &egui::Context, state: &AppState);

    fn set_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, state: &AppState);
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test() {}
}
