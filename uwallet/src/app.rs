use super::activity::{
    common::{*},
    home::{*},
    interface::{*},
    setting::{*},
    transfer::{*},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WalletApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    page: Page,
}

impl Default for WalletApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            page: Page::Home,
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize)]
enum Page {
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
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.common_page(ctx, _frame);
        match self.page {
            Page::Home => {
                self.home_page(ctx, _frame)
            }
            Page::Settings => {
                self.settings_page(ctx, _frame)
            }
            Page::Transfer => {
                self.transfer_page(ctx, _frame)
            }
        }
    }

    fn home_page(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        HomeActivity::on_create(ctx, _frame);
    }


    fn transfer_page(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TransferActivity::on_create(ctx, _frame);
    }


    fn settings_page(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        SettingActivity::on_create(ctx, _frame);
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

