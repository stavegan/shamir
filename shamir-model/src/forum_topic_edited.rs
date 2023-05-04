use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicEdited {
    pub name: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}
