use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatAdministratorRights {
    pub is_anonymous: bool,
    pub can_manage_chat: bool,
    pub can_delete_messages: bool,
    pub can_manage_video_chats: bool,
    pub can_restrict_members: bool,
    pub can_promote_members: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

impl ChatAdministratorRights {
    pub fn from(
        is_anonymous: bool,
        can_manage_chat: bool,
        can_delete_messages: bool,
        can_manage_video_chats: bool,
        can_restrict_members: bool,
        can_promote_members: bool,
        can_change_info: bool,
        can_invite_users: bool,
    ) -> Self {
        Self {
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
        }
    }
}
