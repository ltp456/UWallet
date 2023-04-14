use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use log::{debug, error, info};
use coreui::{
    executor::Executor,
    lifecycle::{ActName, Lifecycle, LifecycleManager},
    state::AppState,
    IActivity,IView,
    egui,eframe,
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
        eframe::set_value(storage, eframe::APP_KEY, &self.app.state.0);
    }
}


impl WalletApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //https://rpc.polkadot.io
        let client = Arc::new(polkadot::client::Client::new(String::from("http://127.0.0.1:9933")));
        let (sender, receiver) = std::sync::mpsc::channel::<ActName>();
        let executor = Arc::new(Executor::new());
        let mut app_state = AppState::new();
        if let Some(storage) = cc.storage {
            app_state = AppState(eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default());
        }
        let mut app = coreui::app::App::new(receiver, app_state);
        app.boot_act(&ActName::new("welcome"), WelcomeActivity::new(sender.clone()));
        //app.boot_act(&ActName::new("temp"), template::TemplateActivity::new(cc.egui_ctx.clone(), &app.navigate_sender, executor.clone()));
        app.register(&ActName::new("password"), PasswordActivity::new(cc.egui_ctx.clone(), sender.clone()));
        app.register(&ActName::new("phrase"), PhraseActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone()));
        app.register(&ActName::new("transfer"), TransferActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone(), client.clone()));
        app.register(&ActName::new("setting"), SettingActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone()));
        app.register(&ActName::new("home"), HomeActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone(), client.clone()));
        Self {
            app
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test() {}
}
