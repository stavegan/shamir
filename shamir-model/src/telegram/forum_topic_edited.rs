use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicEdited {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl ForumTopicEdited {
    pub fn from() -> Self {
        Self {
            name: None,
            icon_custom_emoji_id: None,
        }
    }
}
