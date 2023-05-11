use crate::telegram::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfilePhotos {
    pub total_count: u64,
    pub photos: Vec<Vec<Box<PhotoSize>>>,
}
