use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SwitchInlineQueryChosenChat {
    pub query: Option<String>,
    pub allow_user_chats: Option<bool>,
    pub allow_bot_chats: Option<bool>,
    pub allow_group_chats: Option<bool>,
    pub allow_channel_chats: Option<bool>,
}
