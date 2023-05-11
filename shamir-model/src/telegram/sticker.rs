use crate::telegram::file::File;
use crate::telegram::mask_position::MaskPosition;
use crate::telegram::photo_size::PhotoSize;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<Box<File>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<Box<MaskPosition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

impl Sticker {
    pub fn from(
        file_id: String,
        file_unique_id: String,
        sticker_type: StickerType,
        width: u64,
        height: u64,
        is_animated: bool,
        is_video: bool,
    ) -> Self {
        Self {
            file_id,
            file_unique_id,
            sticker_type,
            width,
            height,
            is_animated,
            is_video,
            thumbnail: None,
            emoji: None,
            set_name: None,
            premium_animation: None,
            mask_position: None,
            custom_emoji_id: None,
            needs_repainting: None,
            file_size: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StickerType {
    Regular,
    Mask,
    CustomEmoji,
}

impl StickerType {
    pub fn regular() -> Self {
        Self::Regular
    }

    pub fn mask() -> Self {
        Self::Mask
    }

    pub fn custom_emoji() -> Self {
        Self::CustomEmoji
    }
}
