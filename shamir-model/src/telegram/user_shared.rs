use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserShared {
    pub request_id: u64,
    pub user_id: u64,
}
