use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}
