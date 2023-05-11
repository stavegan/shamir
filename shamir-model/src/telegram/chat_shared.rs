use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatShared {
    pub request_id: u64,
    pub chat_id: u64,
}

impl ChatShared {
    pub fn from(request_id: u64, chat_id: u64) -> Self {
        Self {
            request_id,
            chat_id,
        }
    }
}
