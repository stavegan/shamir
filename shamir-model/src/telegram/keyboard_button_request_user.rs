use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButtonRequestUser {
    pub request_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
}
