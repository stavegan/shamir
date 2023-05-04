use crate::chat::Chat;
use crate::chat_invite_link::ChatInviteLink;
use crate::chat_member::ChatMember;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMemberUpdated {
    pub chat: Box<Chat>,
    pub from: Box<User>,
    pub date: u64,
    pub old_chat_member: Box<ChatMember>,
    pub new_chat_member: Box<ChatMember>,
    pub invite_link: Option<Box<ChatInviteLink>>,
    pub via_chat_folder_invite_link: Option<bool>,
}
