#![warn(clippy::all, rust_2018_idioms)]


use std::sync::Arc;
use std::sync::mpsc::Sender;

use anyhow::{anyhow, Result};
use serde::{Serialize, Serializer};

use state::AppState;

use crate::lifecycle::ActName;

pub mod lifecycle;
pub mod executor;
pub mod state;
pub mod app;


pub struct Navigate(Sender<ActName>);

impl Navigate {
    pub fn navigate(&self, act_name: ActName) -> Result<()> {
        self.0.send(act_name).map_err(|e| anyhow!("{}",e))
    }
}
