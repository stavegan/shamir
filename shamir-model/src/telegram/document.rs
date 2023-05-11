use crate::telegram::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

impl Document {
    pub fn from(file_id: String, file_unique_id: String) -> Self {
        Self {
            file_id,
            file_unique_id,
            thumbnail: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }
}
