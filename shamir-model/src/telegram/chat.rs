use crate::telegram::chat_location::ChatLocation;
use crate::telegram::chat_permissions::ChatPermissions;
use crate::telegram::chat_photo::ChatPhoto;
use crate::telegram::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: u64,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Box<ChatPhoto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<ChatPermissions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<ChatLocation>>,
}

impl Chat {
    pub fn from(id: u64, chat_type: ChatType) -> Self {
        Self {
            id,
            chat_type,
            title: None,
            username: None,
            first_name: None,
            last_name: None,
            is_forum: None,
            photo: None,
            active_usernames: None,
            emoji_status_custom_emoji_id: None,
            bio: None,
            has_private_forwards: None,
            has_restricted_voice_and_video_messages: None,
            join_to_send_messages: None,
            join_by_request: None,
            description: None,
            invite_link: None,
            pinned_message: None,
            permissions: None,
            slow_mode_delay: None,
            message_auto_delete_time: None,
            has_aggressive_anti_spam_enabled: None,
            has_hidden_members: None,
            has_protected_content: None,
            sticker_set_name: None,
            can_set_sticker_set: None,
            linked_chat_id: None,
            location: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}

impl ChatType {
    pub fn private() -> Self {
        Self::Private
    }

    pub fn group() -> Self {
        Self::Group
    }

    pub fn supergroup() -> Self {
        Self::Supergroup
    }

    pub fn channel() -> Self {
        Self::Channel
    }
}
