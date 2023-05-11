use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BotName {
    pub name: String,
}

impl BotName {
    pub fn from(name: String) -> Self {
        Self { name }
    }
}
