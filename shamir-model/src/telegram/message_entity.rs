use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub message_entity_type: MessageEntityType,
    pub offset: u64,
    pub length: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

impl MessageEntity {
    pub fn from(message_entity_type: MessageEntityType, offset: u64, length: u64) -> Self {
        Self {
            message_entity_type,
            offset,
            length,
            url: None,
            user: None,
            language: None,
            custom_emoji_id: None,
        }
    }
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

impl MessageEntityType {
    pub fn mention() -> Self {
        Self::Mention
    }

    pub fn hashtag() -> Self {
        Self::Hashtag
    }

    pub fn cashtag() -> Self {
        Self::Cashtag
    }

    pub fn bot_command() -> Self {
        Self::BotCommand
    }

    pub fn url() -> Self {
        Self::Url
    }

    pub fn email() -> Self {
        Self::Email
    }

    pub fn phone_number() -> Self {
        Self::PhoneNumber
    }

    pub fn bold() -> Self {
        Self::Bold
    }

    pub fn italic() -> Self {
        Self::Italic
    }

    pub fn underline() -> Self {
        Self::Underline
    }

    pub fn strikethrough() -> Self {
        Self::Strikethrough
    }

    pub fn spoiler() -> Self {
        Self::Spoiler
    }

    pub fn code() -> Self {
        Self::Code
    }

    pub fn pre() -> Self {
        Self::Pre
    }

    pub fn text_link() -> Self {
        Self::TextLink
    }

    pub fn text_mention() -> Self {
        Self::TextMention
    }

    pub fn custom_emoji() -> Self {
        Self::CustomEmoji
    }
}
