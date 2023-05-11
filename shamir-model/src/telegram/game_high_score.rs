use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameHighScore {
    pub position: u64,
    pub user: Box<User>,
    pub score: u64,
}

impl GameHighScore {
    pub fn from(position: u64, user: Box<User>, score: u64) -> Self {
        Self {
            position,
            user,
            score,
        }
    }
}
