use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u64,
    pub height: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}
