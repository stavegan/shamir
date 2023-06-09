use crate::telegram::chat::Chat;
use crate::telegram::chat_invite_link::ChatInviteLink;
use crate::telegram::chat_member::ChatMember;
use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMemberUpdated {
    pub chat: Box<Chat>,
    pub from: Box<User>,
    pub date: u64,
    pub old_chat_member: Box<ChatMember>,
    pub new_chat_member: Box<ChatMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<Box<ChatInviteLink>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
}

impl ChatMemberUpdated {
    pub fn from(
        chat: Box<Chat>,
        from: Box<User>,
        date: u64,
        old_chat_member: Box<ChatMember>,
        new_chat_member: Box<ChatMember>,
    ) -> Self {
        Self {
            chat,
            from,
            date,
            old_chat_member,
            new_chat_member,
            invite_link: None,
            via_chat_folder_invite_link: None,
        }
    }
}
