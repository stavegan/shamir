use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatStarted;

impl VideoChatStarted {
    pub fn from() -> Self {
        Self
    }
}
