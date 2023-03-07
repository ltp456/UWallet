use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use polkadot::{keys::{*}, rpc::{*}};
use polkadot::rpc::client::Client;


use crate::executor::Executor;

use super::activity::{
    common::{*},
    home::{*},
    setting::{*},
    splash::{*},
    transfer::{*},
    welcome::{*},
};

#[derive(serde::Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub enum Page {
    Splash,
    Welcome,
    Home,
    Settings,
    Transfer,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    pub phrase: String,
    pub current_page: Page,
}

impl State {
    pub fn new() -> Self {
        State {
            phrase: "".to_string(),
            current_page: Page::Splash,
        }
    }
}


pub trait IActivity {
    fn on_create(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WalletApp {
    #[serde(skip)]
    activities: BTreeMap<Page, Arc<Mutex<dyn IActivity>>>,

    #[serde(skip)]
    state: Arc<Mutex<State>>,

    #[serde(skip)]
    client: Arc<Client>,
    #[serde(skip)]
    executor: Executor,
}

impl Default for WalletApp {
    fn default() -> Self {
        let state = Arc::new(Mutex::new(State::new()));
        let client = Arc::new(Client::new("https://rpc.polkadot.io".to_string()));

        let mut activities: BTreeMap<Page, Arc<Mutex<dyn IActivity>>> = BTreeMap::new();
        activities.insert(Page::Splash, Arc::new(Mutex::new(SplashActivity::new(state.clone()))));
        activities.insert(Page::Welcome, Arc::new(Mutex::new(WelcomeActivity::new(state.clone()))));
        activities.insert(Page::Home, Arc::new(Mutex::new(HomeActivity::new())));
        activities.insert(Page::Transfer, Arc::new(Mutex::new(TransferActivity::new(client.clone(), state.clone()))));
        activities.insert(Page::Settings, Arc::new(Mutex::new(SettingActivity::new())));
        Self {
            activities,
            state,
            client,
            executor: Executor::new(),
        }
    }
}


impl eframe::App for WalletApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_view(ctx, _frame)
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}


impl WalletApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }
        Default::default()
    }

    pub fn update_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        println!("update  view ");
        let page = self.state.lock().unwrap().current_page.clone();
        match page {
            Page::Splash => {
                self.activities.get(&Page::Splash).unwrap().lock().unwrap().on_create(ctx, _frame);
            }
            Page::Welcome => {
                self.activities.get(&Page::Welcome).unwrap().lock().unwrap().on_create(ctx, _frame);
            }
            Page::Home=>{
                self.common_page(ctx, _frame);
                self.activities.get(&Page::Home).unwrap().lock().unwrap().on_create(ctx, _frame);
            }
            Page::Transfer => {
                self.common_page(ctx, _frame);
                self.activities.get(&Page::Transfer).unwrap().lock().unwrap().on_create(ctx, _frame);
            }
            Page::Settings=>{
                self.common_page(ctx, _frame);
                self.activities.get(&Page::Settings).unwrap().lock().unwrap().on_create(ctx, _frame);
            }
        }
    }



    fn common_page(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Menu");
            ui.separator();
            if ui.button("Home").clicked() {
                self.state.lock().unwrap().current_page = Page::Home
            }
            ui.separator();
            if ui.button("Transfer").clicked() {
                self.state.lock().unwrap().current_page = Page::Transfer
            }
            ui.separator();
            if ui.button("Settings").clicked() {
                self.state.lock().unwrap().current_page = Page::Settings
            }
            ui.separator();
        });
    }
}

