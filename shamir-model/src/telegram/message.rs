use crate::telegram::animation::Animation;
use crate::telegram::audio::Audio;
use crate::telegram::chat::Chat;
use crate::telegram::chat_shared::ChatShared;
use crate::telegram::contact::Contact;
use crate::telegram::dice::Dice;
use crate::telegram::document::Document;
use crate::telegram::forum_topic_closed::ForumTopicClosed;
use crate::telegram::forum_topic_created::ForumTopicCreated;
use crate::telegram::forum_topic_edited::ForumTopicEdited;
use crate::telegram::forum_topic_reopened::ForumTopicReopened;
use crate::telegram::game::Game;
use crate::telegram::general_forum_topic_hidden::GeneralForumTopicHidden;
use crate::telegram::general_forum_topic_unhidden::GeneralForumTopicUnhidden;
use crate::telegram::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::telegram::invoice::Invoice;
use crate::telegram::location::Location;
use crate::telegram::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
use crate::telegram::message_entity::MessageEntity;
use crate::telegram::passport_data::PassportData;
use crate::telegram::photo_size::PhotoSize;
use crate::telegram::poll::Poll;
use crate::telegram::proximity_alert_triggered::ProximityAlertTriggered;
use crate::telegram::sticker::Sticker;
use crate::telegram::successful_payment::SuccessfulPayment;
use crate::telegram::user::User;
use crate::telegram::user_shared::UserShared;
use crate::telegram::venue::Venue;
use crate::telegram::video::Video;
use crate::telegram::video_chat_ended::VideoChatEnded;
use crate::telegram::video_chat_participants_invited::VideoChatParticipantsInvited;
use crate::telegram::video_chat_scheduled::VideoChatScheduled;
use crate::telegram::video_chat_started::VideoChatStarted;
use crate::telegram::video_note::VideoNote;
use crate::telegram::voice::Voice;
use crate::telegram::web_app_data::WebAppData;
use crate::telegram::write_access_allowed::WriteAccessAllowed;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Box<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Box<Chat>>,
    pub date: u64,
    pub chat: Box<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from: Option<Box<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_chat: Option<Box<Chat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_message_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_sender_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_date: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<Box<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Box<MessageEntity>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<Animation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<Audio>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Box<Document>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<Box<PhotoSize>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Box<Sticker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<Video>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<Box<VideoNote>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Box<Voice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<Box<MessageEntity>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Box<Contact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Box<Dice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Box<Game>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Box<Poll>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Box<Venue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<Location>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<Box<User>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<Box<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<Box<PhotoSize>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<Box<MessageAutoDeleteTimerChanged>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Box<Invoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<Box<SuccessfulPayment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_shared: Option<Box<UserShared>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_shared: Option<Box<ChatShared>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<Box<WriteAccessAllowed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<Box<PassportData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<Box<ProximityAlertTriggered>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<Box<ForumTopicCreated>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<Box<ForumTopicEdited>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<Box<ForumTopicClosed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<Box<ForumTopicReopened>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<Box<GeneralForumTopicHidden>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<Box<GeneralForumTopicUnhidden>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<Box<VideoChatScheduled>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<Box<VideoChatStarted>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<Box<VideoChatEnded>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<Box<VideoChatParticipantsInvited>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<Box<WebAppData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

impl Message {
    pub fn from(message_id: u64, date: u64, chat: Box<Chat>) -> Self {
        Self {
            message_id,
            message_thread_id: None,
            from: None,
            sender_chat: None,
            date,
            chat,
            forward_from: None,
            forward_from_chat: None,
            forward_from_message_id: None,
            forward_signature: None,
            forward_sender_name: None,
            forward_date: None,
            is_topic_message: None,
            is_automatic_forward: None,
            reply_to_message: None,
            via_bot: None,
            edit_date: None,
            has_protected_content: None,
            media_group_id: None,
            author_signature: None,
            text: None,
            entities: None,
            animation: None,
            audio: None,
            document: None,
            photo: None,
            sticker: None,
            video: None,
            video_note: None,
            voice: None,
            caption: None,
            caption_entities: None,
            has_media_spoiler: None,
            contact: None,
            dice: None,
            game: None,
            poll: None,
            venue: None,
            location: None,
            new_chat_members: None,
            left_chat_member: None,
            new_chat_title: None,
            new_chat_photo: None,
            delete_chat_photo: None,
            group_chat_created: None,
            supergroup_chat_created: None,
            channel_chat_created: None,
            message_auto_delete_timer_changed: None,
            migrate_to_chat_id: None,
            migrate_from_chat_id: None,
            pinned_message: None,
            invoice: None,
            successful_payment: None,
            user_shared: None,
            chat_shared: None,
            connected_website: None,
            write_access_allowed: None,
            passport_data: None,
            proximity_alert_triggered: None,
            forum_topic_created: None,
            forum_topic_edited: None,
            forum_topic_closed: None,
            forum_topic_reopened: None,
            general_forum_topic_hidden: None,
            general_forum_topic_unhidden: None,
            video_chat_scheduled: None,
            video_chat_started: None,
            video_chat_ended: None,
            video_chat_participants_invited: None,
            web_app_data: None,
            reply_markup: None,
        }
    }
}
