use crate::location::Location;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: Box<User>,
    pub location: Option<Box<Location>>,
    pub inline_message_id: Option<String>,
    pub query: String,
}
