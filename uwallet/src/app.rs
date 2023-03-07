use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use polkadot::{keys::{*}, rpc::{*}};
use polkadot::rpc::client::Client;

use crate::executor::Executor;

use super::activity::{
    common::{*},
    home::{*},
    interface::{*},
    setting::{*},
    splash::{*},
    transfer::{*},
    welcome::{*},
};





/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WalletApp {


    state: State,
    #[serde(skip)]
    page: Page,
    #[serde(skip)]
    splash_activity: SplashActivity,
    #[serde(skip)]
    welcome_activity: WelcomeActivity,
    #[serde(skip)]
    home_activity: Arc<Mutex<HomeActivity>>,
    #[serde(skip)]
    transfer_activity: Arc<Mutex<TransferActivity>>,
    #[serde(skip)]
    client: Arc<Client>,
    #[serde(skip)]
    executor: Executor,
}

impl Default for WalletApp {
    fn default() -> Self {
        Self {
            page: Page::Splash,
            splash_activity: SplashActivity::new(),
            welcome_activity: WelcomeActivity::new(),
            home_activity: Arc::new(Mutex::new(HomeActivity::new())),
            transfer_activity: Arc::new(Mutex::new(TransferActivity::new())),
            state: State::default(),
            client: Arc::new(Client::new("https://rpc.polkadot.io".to_string())),
            executor: Executor::new(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct State {
    phrase: String,

}


impl State {
    pub fn new() -> Self {
        State {
            phrase: "".to_string()
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize)]
enum Page {
    Splash,
    Welcome,
    Home,
    Settings,
    Transfer,
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
        match self.page {
            Page::Splash => {
                self.splash_page(ctx, _frame);
            }
            Page::Welcome => {
                self.welcome_page(ctx, _frame);
            }
            Page::Home => {
                self.common_page(ctx, _frame);
                self.home_page(ctx, _frame)
            }
            Page::Settings => {
                self.common_page(ctx, _frame);
                self.settings_page(ctx, _frame)
            }
            Page::Transfer => {
                self.common_page(ctx, _frame);
                self.transfer_page(ctx, _frame)
            }
        }
    }

    fn home_page(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let address = Key::address_from_phrase(&self.state.phrase, None);
        let tmp = self.home_activity.clone();
        let client = self.client.clone();
        self.executor.spawn(async move {
            let account_info = client.system_account("16mBaA4BPtJzxLchgbHkimRamd4PjnEpELn2N1TS86Hv3NJ7".to_string()).unwrap();
            tmp.lock().unwrap().set(address, format!("{}", account_info.data.free));
            println!("{}", "dadsdf");
        });
        self.home_activity.lock().unwrap().on_create(ctx, _frame)
    }


    fn transfer_page(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.transfer_activity.lock().unwrap().on_create(ctx, _frame);
        let (amount, dest, submitted) = self.transfer_activity.lock().unwrap().get_info();
        if submitted {
            println!("{} {}", amount, dest);
        }
    }


    fn settings_page(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        SettingActivity::on_create(ctx, _frame);
    }

    fn welcome_page(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.welcome_activity.on_create(ctx, _frame);
        let (confirm, input) = self.welcome_activity.get_status();
        if confirm {
            self.page = Page::Home;
            self.state.phrase = input;
        }
    }


    fn splash_page(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.splash_activity.on_create(ctx, _frame);
        let (input, submit) = self.splash_activity.get_res();
        if submit && input == "abcd" {
            self.page = Page::Welcome;
        }
    }


    fn common_page(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Menu");
            ui.separator();
            if ui.button("Home").clicked() {
                self.page = Page::Home
            }
            ui.separator();
            if ui.button("Transfer").clicked() {
                self.page = Page::Transfer
            }
            ui.separator();
            if ui.button("Settings").clicked() {
                self.page = Page::Settings
            }
            ui.separator();
        });
    }
}

