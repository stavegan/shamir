use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BotShortDescription {
    pub short_description: String,
}

impl BotShortDescription {
    pub fn from(short_description: String) -> Self {
        Self { short_description }
    }
}
