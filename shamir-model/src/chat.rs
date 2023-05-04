use crate::chat_location::ChatLocation;
use crate::chat_permissions::ChatPermissions;
use crate::chat_photo::ChatPhoto;
use crate::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: u64,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
    pub photo: Option<Box<ChatPhoto>>,
    pub active_usernames: Option<Vec<String>>,
    pub emoji_status_custom_emoji_id: Option<String>,
    pub bio: Option<String>,
    pub has_private_forwards: Option<bool>,
    pub has_restricted_voice_and_video_messages: Option<bool>,
    pub join_to_send_messages: Option<bool>,
    pub join_by_request: Option<bool>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub permissions: Option<Box<ChatPermissions>>,
    pub slow_mode_delay: Option<u64>,
    pub message_auto_delete_time: Option<u64>,
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    pub has_hidden_members: Option<bool>,
    pub has_protected_content: Option<bool>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
    pub linked_chat_id: Option<u64>,
    pub location: Option<Box<ChatLocation>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}
