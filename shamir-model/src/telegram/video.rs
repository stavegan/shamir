use crate::telegram::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u64,
    pub height: u64,
    pub duration: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

impl Video {
    pub fn from(
        file_id: String,
        file_unique_id: String,
        width: u64,
        height: u64,
        duration: u64,
    ) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            duration,
            thumbnail: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }
}
