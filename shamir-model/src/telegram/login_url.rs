use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

impl LoginUrl {
    pub fn from(url: String) -> Self {
        Self {
            url,
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        }
    }
}
