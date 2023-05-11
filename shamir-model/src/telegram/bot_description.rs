use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BotDescription {
    pub description: String,
}

impl BotDescription {
    pub fn from(description: String) -> Self {
        Self { description }
    }
}
