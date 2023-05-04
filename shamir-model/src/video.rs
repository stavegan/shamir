use crate::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u64,
    pub height: u64,
    pub duration: u64,
    pub thumbnail: Option<Box<PhotoSize>>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}
