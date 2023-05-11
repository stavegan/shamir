use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebAppInfo {
    pub url: String,
}

impl WebAppInfo {
    pub fn from(url: String) -> Self {
        Self { url }
    }
}
