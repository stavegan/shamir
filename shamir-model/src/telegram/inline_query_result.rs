use crate::telegram::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::telegram::input_message_content::InputMessageContent;
use crate::telegram::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InlineQueryResult {
    Article {
        id: String,
        title: String,
        input_message_content: Box<InputMessageContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        hide_url: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_height: Option<u64>,
    },
    Photo {
        id: String,
        photo_url: String,
        thumbnail_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_height: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    #[serde(rename = "photo")]
    CachedPhoto {
        id: String,
        photo_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    Gif {
        id: String,
        gif_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        gif_width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gif_height: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gif_duration: Option<u64>,
        thumbnail_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_mime_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    #[serde(rename = "gif")]
    CachedGif {
        id: String,
        gif_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif {
        id: String,
        mpeg4_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        mpeg4_width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mpeg4_height: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mpeg4_duration: Option<u64>,
        thumbnail_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_mime_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif {
        id: String,
        mpeg4_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    Video {
        id: String,
        video_url: String,
        mime_type: String,
        thumbnail_url: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        video_width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        video_height: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        video_duration: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    #[serde(rename = "video")]
    CachedVideo {
        id: String,
        video_file_id: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    Audio {
        id: String,
        audio_url: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        performer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        audio_duration: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    #[serde(rename = "audio")]
    CachedAudio {
        id: String,
        audio_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    Voice {
        id: String,
        voice_url: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        voice_duration: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    #[serde(rename = "voice")]
    CachedVoice {
        id: String,
        voice_file_id: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    Document {
        id: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        document_url: String,
        mime_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_height: Option<u64>,
    },
    #[serde(rename = "document")]
    CachedDocument {
        id: String,
        title: String,
        document_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
    Location {
        id: String,
        latitude: f64,
        longitude: f64,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        horizontal_accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        live_period: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        heading: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        proximity_alert_radius: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_height: Option<u64>,
    },
    Venue {
        id: String,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_height: Option<u64>,
    },
    Contact {
        id: String,
        phone_number: String,
        first_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        vcard: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_height: Option<u64>,
    },
    Game {
        id: String,
        game_short_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
    },
    #[serde(rename = "sticker")]
    CachedSticker {
        id: String,
        sticker_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<Box<InlineKeyboardMarkup>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<Box<InputMessageContent>>,
    },
}
