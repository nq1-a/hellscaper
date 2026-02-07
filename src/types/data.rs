use std::collections::HashMap;
use std::sync::Mutex;

use serde::{Serialize, Deserialize};

use crate::types::arc::Campaign;

#[derive(Default, Serialize, Deserialize)]
pub struct Data {
    pub campaigns: Mutex<HashMap<String, Campaign>>,
    pub ready: Mutex<bool>,
}
