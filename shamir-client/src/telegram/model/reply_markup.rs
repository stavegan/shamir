use serde::Serialize;
use shamir_model::telegram::ForceReply;
use shamir_model::telegram::InlineKeyboardMarkup;
use shamir_model::telegram::ReplyKeyboardMarkup;
use shamir_model::telegram::ReplyKeyboardRemove;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(Box<InlineKeyboardMarkup>),
    ReplyKeyboardMarkup(Box<ReplyKeyboardMarkup>),
    ReplyKeyboardRemove(Box<ReplyKeyboardRemove>),
    ForceReply(Box<ForceReply>),
}

impl From<Box<InlineKeyboardMarkup>> for ReplyMarkup {
    fn from(inline_keyboard_markup: Box<InlineKeyboardMarkup>) -> Self {
        ReplyMarkup::InlineKeyboardMarkup(inline_keyboard_markup)
    }
}

impl From<Box<ReplyKeyboardMarkup>> for ReplyMarkup {
    fn from(reply_keyboard_markup: Box<ReplyKeyboardMarkup>) -> Self {
        ReplyMarkup::ReplyKeyboardMarkup(reply_keyboard_markup)
    }
}

impl From<Box<ReplyKeyboardRemove>> for ReplyMarkup {
    fn from(reply_keyboard_remove: Box<ReplyKeyboardRemove>) -> Self {
        ReplyMarkup::ReplyKeyboardRemove(reply_keyboard_remove)
    }
}

impl From<Box<ForceReply>> for ReplyMarkup {
    fn from(force_reply: Box<ForceReply>) -> Self {
        ReplyMarkup::ForceReply(force_reply)
    }
}
