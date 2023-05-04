use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: u64,
    pub icon_custom_emoji_id: Option<String>,
}
