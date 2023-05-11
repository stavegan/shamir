use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}
