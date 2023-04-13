
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

//#[derive(serde::Deserialize, serde::Serialize,Default)]
pub struct AppState(pub Arc<Mutex<HashMap<String, String>>>);


impl AppState {
    pub fn new() -> Self {
        AppState(Arc::new(Mutex::new(HashMap::<String, String>::new())))
    }

    pub fn set_value(&self, key: String, value: String) {
        self.0.lock().unwrap().insert(key, value);
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        self.0.lock().unwrap().get(key).cloned()
    }

    pub fn exists(&self, key: &str) -> bool {
        self.0.lock().unwrap().contains_key(key)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let state = AppState(Arc::new(Mutex::new(HashMap::<String, String>::new())));
        state.set_value("a".to_owned(), "b".to_owned());
        let option = state.get_value("a").unwrap();
        println!("{}", option);
    }
}