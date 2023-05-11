use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PollOption {
    pub text: String,
    pub voter_count: u64,
}

impl PollOption {
    pub fn from(text: String, voter_count: u64) -> Self {
        Self { text, voter_count }
    }
}
