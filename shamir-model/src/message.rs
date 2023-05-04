use crate::animation::Animation;
use crate::audio::Audio;
use crate::chat::Chat;
use crate::chat_shared::ChatShared;
use crate::contact::Contact;
use crate::dice::Dice;
use crate::document::Document;
use crate::forum_topic_closed::ForumTopicClosed;
use crate::forum_topic_created::ForumTopicCreated;
use crate::forum_topic_edited::ForumTopicEdited;
use crate::forum_topic_reopened::ForumTopicReopened;
use crate::game::Game;
use crate::general_forum_topic_hidden::GeneralForumTopicHidden;
use crate::general_forum_topic_unhidden::GeneralForumTopicUnhidden;
use crate::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::invoice::Invoice;
use crate::location::Location;
use crate::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
use crate::message_entity::MessageEntity;
use crate::passport_data::PassportData;
use crate::photo_size::PhotoSize;
use crate::poll::Poll;
use crate::proximity_alert_triggered::ProximityAlertTriggered;
use crate::sticker::Sticker;
use crate::successful_payment::SuccessfulPayment;
use crate::user::User;
use crate::user_shared::UserShared;
use crate::venue::Venue;
use crate::video::Video;
use crate::video_chat_ended::VideoChatEnded;
use crate::video_chat_participants_invited::VideoChatParticipantsInvited;
use crate::video_chat_scheduled::VideoChatScheduled;
use crate::video_chat_started::VideoChatStarted;
use crate::video_note::VideoNote;
use crate::voice::Voice;
use crate::web_app_data::WebAppData;
use crate::write_access_allowed::WriteAccessAllowed;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: u64,
    pub message_thread_id: Option<u64>,
    pub from: Option<Box<User>>,
    pub sender_chat: Option<Box<Chat>>,
    pub date: u64,
    pub chat: Box<Chat>,
    pub forward_from: Option<Box<User>>,
    pub forward_from_chat: Option<Box<Chat>>,
    pub forward_from_message_id: Option<u64>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    pub forward_date: Option<u64>,
    pub is_topic_message: Option<bool>,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub via_bot: Option<Box<User>>,
    pub edit_date: Option<u64>,
    pub has_protected_content: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<Box<MessageEntity>>>,
    pub animation: Option<Box<Animation>>,
    pub audio: Option<Box<Audio>>,
    pub document: Option<Box<Document>>,
    pub photo: Option<Vec<Box<PhotoSize>>>,
    pub sticker: Option<Box<Sticker>>,
    pub video: Option<Box<Video>>,
    pub video_note: Option<Box<VideoNote>>,
    pub voice: Option<Box<Voice>>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<Box<MessageEntity>>>,
    pub has_media_spoiler: Option<bool>,
    pub contact: Option<Box<Contact>>,
    pub dice: Option<Box<Dice>>,
    pub game: Option<Box<Game>>,
    pub poll: Option<Box<Poll>>,
    pub venue: Option<Box<Venue>>,
    pub location: Option<Box<Location>>,
    pub new_chat_members: Option<Vec<Box<User>>>,
    pub left_chat_member: Option<Box<User>>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<Box<PhotoSize>>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub message_auto_delete_timer_changed: Option<Box<MessageAutoDeleteTimerChanged>>,
    pub migrate_to_chat_id: Option<u64>,
    pub migrate_from_chat_id: Option<u64>,
    pub pinned_message: Option<Box<Message>>,
    pub invoice: Option<Box<Invoice>>,
    pub successful_payment: Option<Box<SuccessfulPayment>>,
    pub user_shared: Option<Box<UserShared>>,
    pub chat_shared: Option<Box<ChatShared>>,
    pub connected_website: Option<String>,
    pub write_access_allowed: Option<Box<WriteAccessAllowed>>,
    pub passport_data: Option<Box<PassportData>>,
    pub proximity_alert_triggered: Option<Box<ProximityAlertTriggered>>,
    pub forum_topic_created: Option<Box<ForumTopicCreated>>,
    pub forum_topic_edited: Option<Box<ForumTopicEdited>>,
    pub forum_topic_closed: Option<Box<ForumTopicClosed>>,
    pub forum_topic_reopened: Option<Box<ForumTopicReopened>>,
    pub general_forum_topic_hidden: Option<Box<GeneralForumTopicHidden>>,
    pub general_forum_topic_unhidden: Option<Box<GeneralForumTopicUnhidden>>,
    pub video_chat_scheduled: Option<Box<VideoChatScheduled>>,
    pub video_chat_started: Option<Box<VideoChatStarted>>,
    pub video_chat_ended: Option<Box<VideoChatEnded>>,
    pub video_chat_participants_invited: Option<Box<VideoChatParticipantsInvited>>,
    pub web_app_data: Option<Box<WebAppData>>,
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}
