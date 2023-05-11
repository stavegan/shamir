use crate::telegram::mask_position::MaskPosition;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InputSticker {
    pub sticker: String,
    pub emoji_list: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<Box<MaskPosition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

impl InputSticker {
    pub fn from(sticker: String, emoji_list: Vec<String>) -> Self {
        Self {
            sticker,
            emoji_list,
            mask_position: None,
            keywords: None,
        }
    }
}
