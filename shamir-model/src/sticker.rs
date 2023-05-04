use crate::file::File;
use crate::mask_position::MaskPosition;
use crate::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    pub width: u64,
    pub height: u64,
    pub is_animated: bool,
    pub is_video: bool,
    pub thumbnail: Option<Box<PhotoSize>>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub premium_animation: Option<Box<File>>,
    pub mask_position: Option<Box<MaskPosition>>,
    pub custom_emoji_id: Option<String>,
    pub needs_repainting: Option<bool>,
    pub file_size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StickerType {
    Regular,
    Mask,
    CustomEmoji,
}
