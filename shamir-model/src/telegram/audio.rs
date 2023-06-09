use crate::telegram::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
}

impl Audio {
    pub fn from(file_id: String, file_unique_id: String, duration: u64) -> Self {
        Self {
            file_id,
            file_unique_id,
            duration,
            performer: None,
            title: None,
            file_name: None,
            mime_type: None,
            file_size: None,
            thumbnail: None,
        }
    }
}
