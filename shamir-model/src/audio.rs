use crate::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: u64,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
    pub thumbnail: Option<Box<PhotoSize>>,
}
