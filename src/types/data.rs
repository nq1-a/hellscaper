use std::collections::HashMap;
use std::fs::read_to_string as read_fs;
use std::sync::Mutex;

use serde::{Serialize, Deserialize};

use crate::types::arc::Campaign;

#[derive(Default, Serialize, Deserialize)]
pub struct Data {
    pub campaigns: Mutex<HashMap<String, Campaign>>,
    pub config: HashMap<String, String>,
    pub ready: Mutex<bool>,
}

impl Data {
    pub fn load_cfg(&mut self, path: &str) {
        self.config = toml::from_str(&read_fs(path).unwrap()).unwrap();
    }
}
