use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserShared {
    pub request_id: u64,
    pub user_id: u64,
}

impl UserShared {
    pub fn from(request_id: u64, user_id: u64) -> Self {
        Self {
            request_id,
            user_id,
        }
    }
}
