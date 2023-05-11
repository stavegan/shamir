use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}

impl WebAppData {
    pub fn from(data: String, button_text: String) -> Self {
        Self { data, button_text }
    }
}
