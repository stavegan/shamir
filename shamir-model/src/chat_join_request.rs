use crate::chat::Chat;
use crate::chat_invite_link::ChatInviteLink;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatJoinRequest {
    pub chat: Box<Chat>,
    pub from: Box<User>,
    pub user_chat_id: u64,
    pub date: u64,
    pub bio: Option<String>,
    pub invite_link: Option<Box<ChatInviteLink>>,
}
