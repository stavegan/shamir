use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatEnded {
    pub duration: u64,
}

impl VideoChatEnded {
    pub fn from(duration: u64) -> Self {
        Self { duration }
    }
}
