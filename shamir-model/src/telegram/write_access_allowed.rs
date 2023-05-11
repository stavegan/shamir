use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteAccessAllowed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<String>,
}

impl WriteAccessAllowed {
    pub fn from() -> Self {
        Self { web_app_name: None }
    }
}
