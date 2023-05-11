use crate::telegram::keyboard_button_poll_type::KeyboardButtonPollType;
use crate::telegram::keyboard_button_request_chat::KeyboardButtonRequestChat;
use crate::telegram::keyboard_button_request_user::KeyboardButtonRequestUser;
use crate::telegram::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_user: Option<Box<KeyboardButtonRequestUser>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<Box<KeyboardButtonRequestChat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<Box<KeyboardButtonPollType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<Box<WebAppInfo>>,
}
