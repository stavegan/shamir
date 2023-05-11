use crate::telegram::photo_size::PhotoSize;
use crate::telegram::sticker::{Sticker, StickerType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub sticker_type: StickerType,
    pub is_animated: bool,
    pub is_video: bool,
    pub stickers: Vec<Box<Sticker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<PhotoSize>>,
}
