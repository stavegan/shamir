use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatScheduled {
    pub start_date: u64,
}

impl VideoChatScheduled {
    pub fn from(start_date: u64) -> Self {
        Self { start_date }
    }
}
