use crate::telegram::model::reply_markup::ReplyMarkup;
use serde::Serialize;
use shamir_model::telegram::MessageEntity;

#[derive(Serialize, Debug)]
pub struct SendMessageRequest {
    pub chat_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<u64>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Box<MessageEntity>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
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

impl SendMessageRequest {
    pub fn from(chat_id: u64, text: String) -> Self {
        Self {
            chat_id,
            message_thread_id: None,
            text,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}
