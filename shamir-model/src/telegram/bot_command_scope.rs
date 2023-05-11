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

impl BotCommandScope {
    pub fn default() -> Self {
        Self::Default
    }

    pub fn all_private_chats() -> Self {
        Self::AllPrivateChats
    }

    pub fn all_group_chats() -> Self {
        Self::AllGroupChats
    }

    pub fn all_chat_administrators() -> Self {
        Self::AllChatAdministrators
    }

    pub fn chat(chat_id: u64) -> Self {
        Self::Chat { chat_id }
    }

    pub fn chat_administrators(chat_id: u64) -> Self {
        Self::ChatAdministrators { chat_id }
    }

    pub fn chat_member(chat_id: u64, user_id: u64) -> Self {
        Self::ChatMember { chat_id, user_id }
    }
}
