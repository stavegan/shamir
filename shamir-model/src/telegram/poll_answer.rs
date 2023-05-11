use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PollAnswer {
    pub poll_id: String,
    pub user: Box<User>,
    pub option_ids: Vec<u64>,
}

impl PollAnswer {
    pub fn from(poll_id: String, user: Box<User>, option_ids: Vec<u64>) -> Self {
        Self {
            poll_id,
            user,
            option_ids,
        }
    }
}
