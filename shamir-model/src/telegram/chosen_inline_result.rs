use crate::telegram::location::Location;
use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: Box<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<Location>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub query: String,
}
