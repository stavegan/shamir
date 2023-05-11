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

impl ChatMember {
    pub fn owner(user: Box<User>, is_anonymous: bool) -> Self {
        Self::Owner {
            user,
            is_anonymous,
            custom_title: None,
        }
    }

    pub fn administrator(
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
    ) -> Self {
        Self::Administrator {
            user,
            can_be_edited,
            is_anonymous,
            can_manage_chat,
            can_delete_messages,
            can_manage_video_chats,
            can_restrict_members,
            can_promote_members,
            can_change_info,
            can_invite_users,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
            can_manage_topics: None,
            custom_title: None,
        }
    }

    pub fn member(user: Box<User>) -> Self {
        Self::Member { user }
    }

    pub fn restricted(
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
    ) -> Self {
        Self::Restricted {
            user,
            is_member,
            can_send_messages,
            can_send_audios,
            can_send_documents,
            can_send_photos,
            can_send_videos,
            can_send_video_notes,
            can_send_voice_notes,
            can_send_polls,
            can_send_other_messages,
            can_add_web_page_previews,
            can_change_info,
            can_invite_users,
            can_pin_messages,
            can_manage_topics,
            until_date,
        }
    }

    pub fn left(user: Box<User>) -> Self {
        Self::Left { user }
    }

    pub fn banned(user: Box<User>, until_date: u64) -> Self {
        Self::Banned { user, until_date }
    }
}
