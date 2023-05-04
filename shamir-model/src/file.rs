use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<u64>,
    pub file_path: Option<String>,
}
