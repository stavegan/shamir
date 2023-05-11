use crate::telegram::location::Location;
use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQuery {
    pub id: String,
    pub from: Box<User>,
    pub query: String,
    pub offset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query_chat_type: Option<InlineQueryChatType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

impl InlineQuery {
    pub fn from(id: String, from: Box<User>, query: String, offset: String) -> Self {
        Self {
            id,
            from,
            query,
            offset,
            inline_query_chat_type: None,
            location: None,
        }
    }
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

impl InlineQueryChatType {
    pub fn sender() -> Self {
        Self::Sender
    }

    pub fn private() -> Self {
        Self::Private
    }

    pub fn group() -> Self {
        Self::Group
    }

    pub fn supergroup() -> Self {
        Self::Supergroup
    }

    pub fn channel() -> Self {
        Self::Channel
    }
}
