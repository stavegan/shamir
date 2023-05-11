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

impl ReplyMarkup {
    pub fn inline_keyboard_markup(inline_keyboard_markup: Box<InlineKeyboardMarkup>) -> Self {
        Self::InlineKeyboardMarkup(inline_keyboard_markup)
    }

    pub fn reply_keyboard_markup(reply_keyboard_markup: Box<ReplyKeyboardMarkup>) -> Self {
        Self::ReplyKeyboardMarkup(reply_keyboard_markup)
    }

    pub fn reply_keyboard_remove(reply_keyboard_remove: Box<ReplyKeyboardRemove>) -> Self {
        Self::ReplyKeyboardRemove(reply_keyboard_remove)
    }

    pub fn force_reply(force_reply: Box<ForceReply>) -> Self {
        Self::ForceReply(force_reply)
    }
}
