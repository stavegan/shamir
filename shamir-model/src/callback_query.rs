use crate::message::Message;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackQuery {
    pub id: String,
    pub from: Box<User>,
    pub message: Option<Box<Message>>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}
