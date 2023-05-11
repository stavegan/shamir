use crate::telegram::chat_administrator_rights::ChatAdministratorRights;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButtonRequestChat {
    pub request_id: u64,
    pub chat_is_channel: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<Box<ChatAdministratorRights>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<Box<ChatAdministratorRights>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,
}
