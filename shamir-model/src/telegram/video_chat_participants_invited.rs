use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<Box<User>>,
}

impl VideoChatParticipantsInvited {
    pub fn from(users: Vec<Box<User>>) -> Self {
        Self { users }
    }
}
