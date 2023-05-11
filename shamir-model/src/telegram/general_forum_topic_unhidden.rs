use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralForumTopicUnhidden;

impl GeneralForumTopicUnhidden {
    pub fn from() -> Self {
        Self
    }
}
