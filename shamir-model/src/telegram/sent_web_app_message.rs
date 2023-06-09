use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SentWebAppMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

impl SentWebAppMessage {
    pub fn from() -> Self {
        Self {
            inline_message_id: None,
        }
    }
}
