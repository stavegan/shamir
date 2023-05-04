use crate::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: u64,
    pub duration: u64,
    pub thumbnail: Option<Box<PhotoSize>>,
    pub file_size: Option<u64>,
}
