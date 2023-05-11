use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatShared {
    pub request_id: u64,
    pub chat_id: u64,
}
