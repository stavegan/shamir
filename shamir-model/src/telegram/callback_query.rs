use crate::telegram::message::Message;
use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackQuery {
    pub id: String,
    pub from: Box<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

impl CallbackQuery {
    pub fn from(id: String, from: Box<User>, chat_instance: String) -> Self {
        Self {
            id,
            from,
            message: None,
            inline_message_id: None,
            chat_instance,
            data: None,
            game_short_name: None,
        }
    }
}
