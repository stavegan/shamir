use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BotCommandScope {
    Default,
    AllPrivateChats,
    AllGroupChats,
    AllChatAdministrators,
    Chat { chat_id: u64 },
    ChatAdministrators { chat_id: u64 },
    ChatMember { chat_id: u64, user_id: u64 },
}
