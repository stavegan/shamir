use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}
