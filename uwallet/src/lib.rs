use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Result};
use log::{debug, error, info};

use constants::{
    HOME, PASSWORD, PHRASE, PWD_KEY, SETTING, TRANSFER, WELCOME,
};
use coreui::{
    eframe,
    egui,
    executor::Executor,
    IActivity, IView,
    lifecycle::{ActName, Lifecycle, LifecycleManager}, state::AppState,
};

use crate::{activity::{
    home::HomeActivity,
    password::PasswordActivity,
    phrase::PhraseActivity,
    setting::SettingActivity,
    transfer::TransferActivity,
    welcome::WelcomeActivity,
}};

mod activity;
mod view;
pub mod constants;


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WalletApp {
    app: coreui::app::App,
}


impl eframe::App for WalletApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.app.update_view(ctx, _frame)
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        debug!("app shutdown now");
        let encode_data = self.encrypt_state_data().unwrap();
        eframe::set_value(storage, eframe::APP_KEY, &encode_data);
    }
}


impl WalletApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //https://rpc.polkadot.io
        let client = Arc::new(polkadot::client::Client::new(String::from("http://127.0.0.1:9933")));
        let mut app_state = AppState::new();
        if let Some(storage) = cc.storage {
            app_state.set_encode_data(eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default());
        }
        let mut app = coreui::app::App::new(cc.egui_ctx.clone(), app_state);
        app.boot_act(&ActName::new(WELCOME), WelcomeActivity::new());
        //app.boot_act(&ActName::new("temp"), template::TemplateActivity::new());
        app.register(&ActName::new(PASSWORD), PasswordActivity::new());
        app.register(&ActName::new(PHRASE), PhraseActivity::new());
        app.register(&ActName::new(TRANSFER), TransferActivity::new(cc.egui_ctx.clone(), client.clone()));
        app.register(&ActName::new(SETTING), SettingActivity::new(cc.egui_ctx.clone()));
        app.register(&ActName::new(HOME), HomeActivity::new(cc.egui_ctx.clone(), client.clone()));
        Self {
            app
        }
    }

    pub fn encrypt_state_data(&self) -> Result<String> {
        let map = self.app.state.get_data().unwrap();
        let pwd = self.app.state.get_value(PWD_KEY).unwrap();
        let data = serde_json::to_string(&map).map_err(|e| { anyhow!("{}",e) })?;
        let result = utils::aes::wrapp_encode(data.as_bytes(), pwd.as_bytes()).unwrap();
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test() {}
}
