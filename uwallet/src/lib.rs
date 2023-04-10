#![warn(clippy::all, rust_2018_idioms)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize,Deserialize};
pub use app::WalletApp;

mod app;
mod activity;
mod executor;
mod navigation;
mod view;
mod common;


pub trait IView {
    fn view(&mut self, ui: &mut egui::Ui);
}

pub trait IActivity {
    fn on_create(&mut self, state: &AppState);

    fn on_resume(&mut self, state: &AppState);

    fn on_pause(&mut self, state: &AppState);

    fn set_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, state: &AppState);
}


#[derive(Serialize,Deserialize,Default)]
pub struct AppState(Arc<Mutex<HashMap<String, String>>>);


impl AppState {
    pub fn new() -> Self {
        AppState(Arc::new(Mutex::new(HashMap::<String, String>::new())))
    }

    pub fn set_value(&self, key: String, value: String) {
        self.0.lock().unwrap().insert(key, value);
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        self.0.lock().unwrap().get(key).cloned()
    }

    pub fn exists(&self, key: &str) -> bool {
        self.0.lock().unwrap().contains_key(key)
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use crate::AppState;

    #[test]
    fn test() {
        let state = AppState(Arc::new(Mutex::new(HashMap::<String, String>::new())));
        //state.set_value("a".to_owned(), "b".to_owned());
        let option = state.get_value("a").unwrap();
        println!("{}", option);
    }
}
