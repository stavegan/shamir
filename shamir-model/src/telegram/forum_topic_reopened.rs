use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicReopened;

impl ForumTopicReopened {
    pub fn from() -> Self {
        Self
    }
}
