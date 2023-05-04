use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dice {
    pub emoji: String,
    pub value: u64,
}
