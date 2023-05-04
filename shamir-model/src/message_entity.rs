use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub message_entity_type: MessageEntityType,
    pub offset: u64,
    pub length: u64,
    pub url: Option<String>,
    pub user: Option<Box<User>>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MessageEntityType {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
    Pre,
    TextLink,
    TextMention,
    CustomEmoji,
}
