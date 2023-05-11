use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

impl ReplyKeyboardRemove {
    pub fn from(remove_keyboard: bool) -> Self {
        Self {
            remove_keyboard,
            selective: None,
        }
    }
}
