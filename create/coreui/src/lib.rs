#![warn(clippy::all, rust_2018_idioms)]


use std::sync::Arc;
use std::sync::mpsc::Sender;

use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use serde::{Serialize, Serializer};

pub use {eframe, egui};
use state::AppState;

pub mod lifecycle;
pub mod executor;
pub mod state;
pub mod app;

pub trait IView {
    fn view(&mut self, ui: &mut egui::Ui);
}

pub trait IActivity {
    fn on_create(&mut self, ctx: &egui::Context, state: &AppState);

    fn on_resume(&mut self, ctx: &egui::Context, state: &AppState);

    fn on_pause(&mut self, ctx: &egui::Context, state: &AppState);

    fn set_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, state: &AppState);
}
