use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: u64,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}
