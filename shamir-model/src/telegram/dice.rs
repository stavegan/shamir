use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dice {
    pub emoji: String,
    pub value: u64,
}

impl Dice {
    pub fn from(emoji: String, value: u64) -> Self {
        Self { emoji, value }
    }
}
