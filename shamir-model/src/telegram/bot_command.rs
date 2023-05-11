use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

impl BotCommand {
    pub fn from(command: String, description: String) -> Self {
        Self {
            command,
            description,
        }
    }
}
