use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

impl File {
    pub fn from(file_id: String, file_unique_id: String) -> Self {
        Self {
            file_id,
            file_unique_id,
            file_size: None,
            file_path: None,
        }
    }
}
