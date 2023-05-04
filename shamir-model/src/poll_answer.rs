use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PollAnswer {
    pub poll_id: String,
    pub user: Box<User>,
    pub option_ids: Vec<u64>,
}
