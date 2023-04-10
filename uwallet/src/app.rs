use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::Result;
use log::{debug, error, info};
use parking_lot::Mutex;

use crate::{AppState, IActivity};
use crate::activity::home::HomeActivity;
use crate::activity::password::PasswordActivity;
use crate::activity::phrase::PhraseActivity;
use crate::activity::setting::SettingActivity;
use crate::activity::transfer::TransferActivity;
use crate::activity::welcome::WelcomeActivity;
use crate::executor::Executor;
use crate::navigation::{ActivityKey, InnerNavigation, Navigation};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WalletApp {
    activities: HashMap<ActivityKey, Box<dyn IActivity>>,
    navigation: InnerNavigation,
    executor: Arc<Executor>,
    promise: Receiver<ActivityKey>,
    navigate_sender: Sender<ActivityKey>,
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
        eframe::set_value(storage, eframe::APP_KEY, &self.app_state);
    }
}


impl WalletApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //https://rpc.polkadot.io
        let client = Arc::new(polkadot::client::Client::new(String::from("http://127.0.0.1:9933")));
        let (sender, receiver) = std::sync::mpsc::channel::<ActivityKey>();
        let executor = Arc::new(Executor::new());
        let mut app_state = AppState::new();
        if let Some(storage) = cc.storage {
            app_state = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        let mut app = Self {
            activities: HashMap::new(),
            navigation: InnerNavigation::new(),
            executor: executor.clone(),
            promise: receiver,
            navigate_sender: sender.clone(),
            app_state,
        };


        app.boot_activity(ActivityKey::new("welcome"), WelcomeActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone()));
        //app.boot_activity(ActivityKey::new("home"), HomeActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone(), client.clone()));
        app.register(ActivityKey::new("password"), PasswordActivity::new(cc.egui_ctx.clone(), sender.clone()));
        app.register(ActivityKey::new("phrase"), PhraseActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone()));
        app.register(ActivityKey::new("transfer"), TransferActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone(), client.clone()));
        app.register(ActivityKey::new("setting"), SettingActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone()));
        app.register(ActivityKey::new("home"), HomeActivity::new(cc.egui_ctx.clone(), sender.clone(), executor.clone(),client.clone()));
        app
    }

    pub fn register(&mut self, activity_key: ActivityKey, activity: impl IActivity + 'static) {
        self.navigation.register(activity_key.clone()).unwrap();
        self.activities.insert(activity_key, Box::new(activity));
    }

    pub fn boot_activity(&mut self, activity_key: ActivityKey, activity: impl IActivity + 'static) {
        self.navigation.init_activity(activity_key.clone()).unwrap();
        self.activities.insert(activity_key, Box::new(activity));
    }

    pub fn update_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut current_activity_key = self.navigation.get_current_activity().unwrap();
        let current_activity = self.activities.get_mut(&current_activity_key).unwrap();
        current_activity.set_view(ctx, _frame, &self.app_state);
        //todo
        let lifecycle = self.navigation.get_lifecycle(&current_activity_key);
        if lifecycle.on_create {
            current_activity.on_create(&self.app_state);
            lifecycle.on_create = false;
        }
        if lifecycle.on_resume {
            lifecycle.on_resume = false;
            current_activity.on_resume(&self.app_state);
        }

        if let Some(prev_activity) = self.navigation.get_prev_activity() {
            let prev_lifecycle = self.navigation.get_lifecycle(&prev_activity);
            let prev_activity = self.activities.get_mut(&prev_activity).unwrap();
            if prev_lifecycle.on_pause {
                prev_lifecycle.on_pause = false;
                prev_activity.on_pause(&self.app_state);
            }
        }

        if let Ok(result) = self.promise.try_recv() {
            debug!("update activity to {}",result);
            self.navigation.navigate(result);
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