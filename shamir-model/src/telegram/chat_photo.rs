use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}

impl ChatPhoto {
    pub fn from(
        small_file_id: String,
        small_file_unique_id: String,
        big_file_id: String,
        big_file_unique_id: String,
    ) -> Self {
        Self {
            small_file_id,
            small_file_unique_id,
            big_file_id,
            big_file_unique_id,
        }
    }
}
