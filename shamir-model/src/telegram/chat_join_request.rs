use crate::telegram::chat::Chat;
use crate::telegram::chat_invite_link::ChatInviteLink;
use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatJoinRequest {
    pub chat: Box<Chat>,
    pub from: Box<User>,
    pub user_chat_id: u64,
    pub date: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<Box<ChatInviteLink>>,
}

impl ChatJoinRequest {
    pub fn from(chat: Box<Chat>, from: Box<User>, user_chat_id: u64, date: u64) -> Self {
        Self {
            chat,
            from,
            user_chat_id,
            date,
            bio: None,
            invite_link: None,
        }
    }
}
