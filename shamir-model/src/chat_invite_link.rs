use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: Box<User>,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    pub name: Option<String>,
    pub expire_date: Option<u64>,
    pub member_limit: Option<u64>,
    pub pending_join_request_count: Option<u64>,
}
