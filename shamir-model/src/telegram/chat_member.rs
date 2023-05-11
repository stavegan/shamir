use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ChatMember {
    #[serde(rename = "creator")]
    Owner {
        user: Box<User>,
        is_anonymous: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        custom_title: Option<String>,
    },
    Administrator {
        user: Box<User>,
        can_be_edited: bool,
        is_anonymous: bool,
        can_manage_chat: bool,
        can_delete_messages: bool,
        can_manage_video_chats: bool,
        can_restrict_members: bool,
        can_promote_members: bool,
        can_change_info: bool,
        can_invite_users: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_post_messages: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_edit_messages: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_pin_messages: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_manage_topics: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        custom_title: Option<String>,
    },
    Member {
        user: Box<User>,
    },
    Restricted {
        user: Box<User>,
        is_member: bool,
        can_send_messages: bool,
        can_send_audios: bool,
        can_send_documents: bool,
        can_send_photos: bool,
        can_send_videos: bool,
        can_send_video_notes: bool,
        can_send_voice_notes: bool,
        can_send_polls: bool,
        can_send_other_messages: bool,
        can_add_web_page_previews: bool,
        can_change_info: bool,
        can_invite_users: bool,
        can_pin_messages: bool,
        can_manage_topics: bool,
        until_date: u64,
    },
    Left {
        user: Box<User>,
    },
    #[serde(rename = "kicked")]
    Banned {
        user: Box<User>,
        until_date: u64,
    },
}
