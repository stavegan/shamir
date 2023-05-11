use crate::telegram::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputMedia {
    Photo {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
    },
    Video {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        supports_streaming: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
    },
    Animation {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
    },
    Audio {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        performer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    Document {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_content_type_detection: Option<bool>,
    },
}

impl InputMedia {
    pub fn photo(media: String) -> Self {
        Self::Photo {
            media,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
        }
    }

    pub fn video(media: String) -> Self {
        Self::Video {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
            has_spoiler: None,
        }
    }

    pub fn animation(media: String) -> Self {
        Self::Animation {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
            has_spoiler: None,
        }
    }

    pub fn audio(media: String) -> Self {
        Self::Audio {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
        }
    }

    pub fn document(media: String) -> Self {
        Self::Document {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
        }
    }
}
