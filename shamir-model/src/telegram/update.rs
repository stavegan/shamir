use crate::telegram::callback_query::CallbackQuery;
use crate::telegram::chat_join_request::ChatJoinRequest;
use crate::telegram::chat_member_updated::ChatMemberUpdated;
use crate::telegram::chosen_inline_result::ChosenInlineResult;
use crate::telegram::inline_query::InlineQuery;
use crate::telegram::message::Message;
use crate::telegram::poll::Poll;
use crate::telegram::poll_answer::PollAnswer;
use crate::telegram::pre_checkout_query::PreCheckoutQuery;
use crate::telegram::shipping_query::ShippingQuery;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: u64,
    #[serde(flatten)]
    pub value: UpdateValue,
}

impl Update {
    pub fn from(update_id: u64, value: UpdateValue) -> Self {
        Self { update_id, value }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum UpdateValue {
    Message {
        message: Message,
    },
    EditedMessage {
        edited_message: Message,
    },
    ChannelPost {
        channel_post: Message,
    },
    EditedChannelPost {
        edited_channel_post: Message,
    },
    InlineQuery {
        inline_query: InlineQuery,
    },
    ChosenInlineResult {
        chosen_inline_result: ChosenInlineResult,
    },
    CallbackQuery {
        callback_query: CallbackQuery,
    },
    ShippingQuery {
        shipping_query: ShippingQuery,
    },
    PreCheckoutQuery {
        pre_checkout_query: PreCheckoutQuery,
    },
    Poll {
        poll: Poll,
    },
    PollAnswer {
        poll_answer: PollAnswer,
    },
    MyChatMember {
        my_chat_member: ChatMemberUpdated,
    },
    ChatMember {
        chat_member: ChatMemberUpdated,
    },
    ChatJoinRequest {
        chat_join_request: ChatJoinRequest,
    },
}

impl UpdateValue {
    pub fn message(message: Message) -> Self {
        Self::Message { message }
    }

    pub fn edited_message(edited_message: Message) -> Self {
        Self::EditedMessage { edited_message }
    }

    pub fn channel_post(channel_post: Message) -> Self {
        Self::ChannelPost { channel_post }
    }

    pub fn edited_channel_post(edited_channel_post: Message) -> Self {
        Self::EditedChannelPost {
            edited_channel_post,
        }
    }

    pub fn inline_query(inline_query: InlineQuery) -> Self {
        Self::InlineQuery { inline_query }
    }

    pub fn chosen_inline_result(chosen_inline_result: ChosenInlineResult) -> Self {
        Self::ChosenInlineResult {
            chosen_inline_result,
        }
    }

    pub fn callback_query(callback_query: CallbackQuery) -> Self {
        Self::CallbackQuery { callback_query }
    }

    pub fn shipping_query(shipping_query: ShippingQuery) -> Self {
        Self::ShippingQuery { shipping_query }
    }

    pub fn pre_checkout_query(pre_checkout_query: PreCheckoutQuery) -> Self {
        Self::PreCheckoutQuery { pre_checkout_query }
    }

    pub fn poll(poll: Poll) -> Self {
        Self::Poll { poll }
    }

    pub fn poll_answer(poll_answer: PollAnswer) -> Self {
        Self::PollAnswer { poll_answer }
    }

    pub fn my_chat_member(my_chat_member: ChatMemberUpdated) -> Self {
        Self::MyChatMember { my_chat_member }
    }

    pub fn chat_member(chat_member: ChatMemberUpdated) -> Self {
        Self::ChatMember { chat_member }
    }

    pub fn chat_join_request(chat_join_request: ChatJoinRequest) -> Self {
        Self::ChatJoinRequest { chat_join_request }
    }
}
