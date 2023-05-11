use crate::telegram::poll::PollType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub poll_type: Option<PollType>,
}

impl KeyboardButtonPollType {
    pub fn from() -> Self {
        Self { poll_type: None }
    }
}
