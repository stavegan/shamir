use crate::callback_game::CallbackGame;
use crate::login_url::LoginUrl;
use crate::switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;
use crate::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    pub web_app: Option<Box<WebAppInfo>>,
    pub login_url: Option<Box<LoginUrl>>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_chosen_chat: Option<Box<SwitchInlineQueryChosenChat>>,
    pub callback_game: Option<Box<CallbackGame>>,
    pub pay: Option<bool>,
}
