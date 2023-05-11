use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageId {
    pub message_id: u64,
}

impl MessageId {
    pub fn from(message_id: u64) -> Self {
        Self { message_id }
    }
}
