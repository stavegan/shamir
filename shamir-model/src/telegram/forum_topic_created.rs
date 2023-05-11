use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl ForumTopicCreated {
    pub fn from(name: String, icon_color: u64) -> Self {
        Self {
            name,
            icon_color,
            icon_custom_emoji_id: None,
        }
    }
}
