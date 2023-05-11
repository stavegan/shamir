use crate::telegram::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: u64,
    pub duration: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

impl VideoNote {
    pub fn from(file_id: String, file_unique_id: String, length: u64, duration: u64) -> Self {
        Self {
            file_id,
            file_unique_id,
            length,
            duration,
            thumbnail: None,
            file_size: None,
        }
    }
}
