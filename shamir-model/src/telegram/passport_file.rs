use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: u64,
    pub file_date: u64,
}

impl PassportFile {
    pub fn from(file_id: String, file_unique_id: String, file_size: u64, file_date: u64) -> Self {
        Self {
            file_id,
            file_unique_id,
            file_size,
            file_date,
        }
    }
}
