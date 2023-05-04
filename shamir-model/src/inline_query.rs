use crate::location::Location;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQuery {
    pub id: String,
    pub from: Box<User>,
    pub query: String,
    pub offset: String,
    pub inline_query_chat_type: Option<InlineQueryChatType>,
    pub location: Option<Location>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InlineQueryChatType {
    Sender,
    Private,
    Group,
    Supergroup,
    Channel,
}
