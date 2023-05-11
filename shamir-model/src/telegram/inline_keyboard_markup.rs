use crate::telegram::inline_keyboard_button::InlineKeyboardButton;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<Box<InlineKeyboardButton>>>,
}
