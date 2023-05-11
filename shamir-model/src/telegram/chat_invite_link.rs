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

impl ChatInviteLink {
    pub fn from(
        invite_link: String,
        creator: Box<User>,
        creates_join_request: bool,
        is_primary: bool,
        is_revoked: bool,
    ) -> Self {
        Self {
            invite_link,
            creator,
            creates_join_request,
            is_primary,
            is_revoked,
            name: None,
            expire_date: None,
            member_limit: None,
            pending_join_request_count: None,
        }
    }
}
