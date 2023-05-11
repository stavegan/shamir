use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PollOption {
    pub text: String,
    pub voter_count: u64,
}
