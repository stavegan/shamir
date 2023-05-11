use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

impl Voice {
    pub fn from(file_id: String, file_unique_id: String, duration: u64) -> Self {
        Self {
            file_id,
            file_unique_id,
            duration,
            mime_type: None,
            file_size: None,
        }
    }
}
