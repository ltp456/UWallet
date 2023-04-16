use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use serde::{Deserialize, Serialize};

//#[derive(serde::Deserialize, serde::Serialize,Default)]
pub struct AppState(Arc<Mutex<State>>);

impl AppState {
    pub fn new() -> Self {
        AppState(Arc::new(Mutex::new(State::new())))
    }

    pub fn set_value(&self, key: &str, value: &str) {
        self.0.lock().unwrap().hashmap.insert(key.to_owned(), value.to_owned());
    }

    pub fn exists(&self, key: &str) -> bool {
        self.0.lock().unwrap().hashmap.contains_key(key)
    }
    pub fn get_value(&self, key: &str) -> Option<String> {
        self.0.lock().unwrap().hashmap.get(key).cloned()
    }

    pub fn get_data(&self) -> Option<HashMap<String, String>> {
        Some(self.0.lock().unwrap().hashmap.clone())
    }
    pub fn init_data(&self, data: &[u8]) {
        self.0.lock().unwrap().reload(data);
    }
    pub fn get_encode_data(&self) -> Result<String> {
        Ok(self.0.lock().unwrap().secret_data.clone())
    }

    pub fn pwd_exists(&self) -> bool {
        self.0.lock().unwrap().secret_data != ""
    }

    pub fn load_data(&self, data: String) {
        self.0.lock().unwrap().secret_data = data;
    }


}


#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip)]
    hashmap: HashMap<String, String>,
    pub secret_data: String,
    pub pwd_hash: String,
}

impl State {
    fn new() -> Self {
        State {
            hashmap: HashMap::new(),
            secret_data: String::new(),
            pwd_hash: String::new(),
        }
    }
    pub fn reload(&mut self, data: &[u8]) {
        let map: HashMap<String, String> = serde_json::from_slice(data).unwrap();
        self.hashmap = map;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}