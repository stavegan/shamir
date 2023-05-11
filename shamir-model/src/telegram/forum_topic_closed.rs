use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForumTopicClosed;

impl ForumTopicClosed {
    pub fn from() -> Self {
        Self
    }
}
