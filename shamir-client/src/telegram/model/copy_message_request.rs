use crate::telegram::model::reply_markup::ReplyMarkup;
use serde::Serialize;
use shamir_model::telegram::MessageEntity;

#[derive(Serialize, Debug)]
pub struct CopyMessageRequest {
    pub chat_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<u64>,
    pub from_chat_id: u64,
    pub message_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<Box<MessageEntity>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl CopyMessageRequest {
    pub fn from(chat_id: u64, from_chat_id: u64, message_id: u64) -> Self {
        CopyMessageRequest {
            chat_id,
            message_thread_id: None,
            from_chat_id,
            message_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}
