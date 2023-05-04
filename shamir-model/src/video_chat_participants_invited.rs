use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<Box<User>>,
}
