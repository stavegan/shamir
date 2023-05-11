use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralForumTopicHidden;

impl GeneralForumTopicHidden {
    pub fn from() -> Self {
        Self
    }
}
