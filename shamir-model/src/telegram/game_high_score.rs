use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameHighScore {
    pub position: u64,
    pub user: Box<User>,
    pub score: u64,
}
