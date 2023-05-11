use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: Box<User>,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<u64>,
}
