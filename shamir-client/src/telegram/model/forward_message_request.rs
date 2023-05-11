use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ForwardMessageRequest {
    pub chat_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<u64>,
    pub from_chat_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    pub message_id: u64,
}

impl ForwardMessageRequest {
    pub fn from(chat_id: u64, from_chat_id: u64, message_id: u64) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            from_chat_id,
            disable_notification: None,
            protect_content: None,
            message_id,
        }
    }
}
