use crate::telegram::callback_game::CallbackGame;
use crate::telegram::login_url::LoginUrl;
use crate::telegram::switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;
use crate::telegram::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<Box<WebAppInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<Box<LoginUrl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<Box<SwitchInlineQueryChosenChat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<Box<CallbackGame>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}
