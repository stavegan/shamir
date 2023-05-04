use crate::inline_keyboard_button::InlineKeyboardButton;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<Box<InlineKeyboardButton>>>,
}
