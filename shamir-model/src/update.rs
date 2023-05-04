use crate::callback_query::CallbackQuery;
use crate::chat_join_request::ChatJoinRequest;
use crate::chat_member_updated::ChatMemberUpdated;
use crate::chosen_inline_result::ChosenInlineResult;
use crate::inline_query::InlineQuery;
use crate::message::Message;
use crate::poll::Poll;
use crate::poll_answer::PollAnswer;
use crate::pre_checkout_query::PreCheckoutQuery;
use crate::shipping_query::ShippingQuery;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: u64,
    #[serde(flatten)]
    pub value: UpdateValue,
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
