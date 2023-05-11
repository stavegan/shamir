use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForceReply {
    pub force_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

impl ForceReply {
    pub fn from(force_reply: bool) -> Self {
        Self {
            force_reply,
            input_field_placeholder: None,
            selective: None,
        }
    }
}
