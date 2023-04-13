use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::Result;
use log::{debug, error, info};
use parking_lot::Mutex;

use coreui::{executor::Executor, lifecycle::Lifecycle, state::AppState};
use coreui::lifecycle::{ActName, StackManager};

use crate::activity::home::HomeActivity;
use crate::activity::password::PasswordActivity;
use crate::activity::phrase::PhraseActivity;
use crate::activity::setting::SettingActivity;
use crate::activity::transfer::TransferActivity;
use crate::activity::welcome::WelcomeActivity;
use crate::IActivity;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WalletApp {
    activities: HashMap<ActName, Box<dyn IActivity>>,
    lifecycle_manager: StackManager,
    executor: Arc<Executor>,
    promise: Receiver<ActName>,
    navigate_sender: Sender<ActName>,
    app_state: AppState,
}


impl eframe::App for WalletApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_view(ctx, _frame)
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        debug!("app shutdown now");
        eframe::set_value(storage, eframe::APP_KEY, &self.app_state.0);
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
        let mut app = Self {
            activities: HashMap::new(),
            lifecycle_manager: StackManager::new(),
            executor: executor.clone(),
            promise: receiver,
            navigate_sender: sender.clone(),
            app_state,
        };


        app.boot_activity(&ActName::new("welcome"), WelcomeActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone()));
        //app.boot_activity(ActivityKey::new("home"), HomeActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone(), client.clone()));
        app.register(&ActName::new("password"), PasswordActivity::new(cc.egui_ctx.clone(), sender.clone()));
        app.register(&ActName::new("phrase"), PhraseActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone()));
        app.register(&ActName::new("transfer"), TransferActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone(), client.clone()));
        app.register(&ActName::new("setting"), SettingActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone()));
        app.register(&ActName::new("home"), HomeActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone(), client.clone()));
        app
    }

    pub fn register(&mut self, activity_key: &ActName, activity: impl IActivity + 'static) {
        self.lifecycle_manager.register(activity_key).unwrap();
        self.activities.insert(activity_key.clone(), Box::new(activity));
    }

    pub fn boot_activity(&mut self, activity_key: &ActName, activity: impl IActivity + 'static) {
        self.lifecycle_manager.boot_act(activity_key).unwrap();
        self.activities.insert(activity_key.clone(), Box::new(activity));
    }

    pub fn update_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (current, prev) = self.lifecycle_manager.current();
        let current_activity = self.activities.get_mut(&current).unwrap();
        current_activity.set_view(ctx, _frame, &self.app_state);

        let lifecycle = self.lifecycle_manager.lifecycle(&current).unwrap();
        if lifecycle.on_create {
            current_activity.on_create(&self.app_state);
        }
        if lifecycle.on_resume {
            current_activity.on_resume(&self.app_state);
        }

        if let Some(prev_activity) = prev {
            let prev_lifecycle = self.lifecycle_manager.lifecycle(&prev_activity).unwrap();
            let prev_activity = self.activities.get_mut(&prev_activity).unwrap();
            if prev_lifecycle.on_pause {
                prev_activity.on_pause(&self.app_state);
            }
        }
        self.lifecycle_manager.reset_lifecycle();
        if let Ok(result) = self.promise.try_recv() {
            debug!("update activity to {}",result);
            self.lifecycle_manager.start_act(result).unwrap();
            ctx.request_repaint();
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}