mod animation;
mod audio;
mod bot_command;
mod bot_command_scope;
mod bot_description;
mod bot_name;
mod bot_short_description;
mod callback_game;
mod callback_query;
mod chat;
mod chat_administrator_rights;
mod chat_invite_link;
mod chat_join_request;
mod chat_location;
mod chat_member;
mod chat_member_updated;
mod chat_permissions;
mod chat_photo;
mod chat_shared;
mod chosen_inline_result;
mod contact;
mod dice;
mod document;
mod encrypted_credentials;
mod encrypted_passport_element;
mod file;
mod force_reply;
mod forum_topic_closed;
mod forum_topic_created;
mod forum_topic_edited;
mod forum_topic_reopened;
mod game;
mod game_high_score;
mod general_forum_topic_hidden;
mod general_forum_topic_unhidden;
mod inline_keyboard_button;
mod inline_keyboard_markup;
mod inline_query;
mod inline_query_result;
mod inline_query_results_button;
mod input_media;
mod input_message_content;
mod input_sticker;
mod invoice;
mod keyboard_button;
mod keyboard_button_poll_type;
mod keyboard_button_request_chat;
mod keyboard_button_request_user;
mod labeled_price;
mod location;
mod login_url;
mod mask_position;
mod menu_button;
mod message;
mod message_auto_delete_timer_changed;
mod message_entity;
mod message_id;
mod order_info;
mod passport_data;
mod passport_element_error;
mod passport_file;
mod photo_size;
mod poll;
mod poll_answer;
mod poll_option;
mod pre_checkout_query;
mod proximity_alert_triggered;
mod reply_keyboard_markup;
mod reply_keyboard_remove;
mod response_parameters;
mod sent_web_app_message;
mod shipping_address;
mod shipping_option;
mod shipping_query;
mod sticker;
mod sticker_set;
mod successful_payment;
mod switch_inline_query_chosen_chat;
mod update;
mod user;
mod user_profile_photos;
mod user_shared;
mod venue;
mod video;
mod video_chat_ended;
mod video_chat_participants_invited;
mod video_chat_scheduled;
mod video_chat_started;
mod video_note;
mod voice;
mod web_app_data;
mod web_app_info;
mod webhook_info;
mod write_access_allowed;

pub use crate::telegram::animation::Animation;
pub use crate::telegram::audio::Audio;
pub use crate::telegram::bot_command::BotCommand;
pub use crate::telegram::bot_command_scope::BotCommandScope;
pub use crate::telegram::bot_description::BotDescription;
pub use crate::telegram::bot_name::BotName;
pub use crate::telegram::bot_short_description::BotShortDescription;
pub use crate::telegram::callback_game::CallbackGame;
pub use crate::telegram::callback_query::CallbackQuery;
pub use crate::telegram::chat::Chat;
pub use crate::telegram::chat::ChatType;
pub use crate::telegram::chat_administrator_rights::ChatAdministratorRights;
pub use crate::telegram::chat_invite_link::ChatInviteLink;
pub use crate::telegram::chat_join_request::ChatJoinRequest;
pub use crate::telegram::chat_location::ChatLocation;
pub use crate::telegram::chat_member::ChatMember;
pub use crate::telegram::chat_member_updated::ChatMemberUpdated;
pub use crate::telegram::chat_permissions::ChatPermissions;
pub use crate::telegram::chat_photo::ChatPhoto;
pub use crate::telegram::chat_shared::ChatShared;
pub use crate::telegram::chosen_inline_result::ChosenInlineResult;
pub use crate::telegram::contact::Contact;
pub use crate::telegram::dice::Dice;
pub use crate::telegram::document::Document;
pub use crate::telegram::encrypted_credentials::EncryptedCredentials;
pub use crate::telegram::encrypted_passport_element::EncryptedPassportElement;
pub use crate::telegram::encrypted_passport_element::EncryptedPassportElementType;
pub use crate::telegram::file::File;
pub use crate::telegram::force_reply::ForceReply;
pub use crate::telegram::forum_topic_closed::ForumTopicClosed;
pub use crate::telegram::forum_topic_created::ForumTopicCreated;
pub use crate::telegram::forum_topic_edited::ForumTopicEdited;
pub use crate::telegram::forum_topic_reopened::ForumTopicReopened;
pub use crate::telegram::game::Game;
pub use crate::telegram::game_high_score::GameHighScore;
pub use crate::telegram::general_forum_topic_hidden::GeneralForumTopicHidden;
pub use crate::telegram::general_forum_topic_unhidden::GeneralForumTopicUnhidden;
pub use crate::telegram::inline_keyboard_button::InlineKeyboardButton;
pub use crate::telegram::inline_keyboard_markup::InlineKeyboardMarkup;
pub use crate::telegram::inline_query::InlineQuery;
pub use crate::telegram::inline_query_result::InlineQueryResult;
pub use crate::telegram::inline_query_results_button::InlineQueryResultsButton;
pub use crate::telegram::input_media::InputMedia;
pub use crate::telegram::input_message_content::InputMessageContent;
pub use crate::telegram::input_sticker::InputSticker;
pub use crate::telegram::invoice::Invoice;
pub use crate::telegram::keyboard_button::KeyboardButton;
pub use crate::telegram::keyboard_button_poll_type::KeyboardButtonPollType;
pub use crate::telegram::keyboard_button_request_chat::KeyboardButtonRequestChat;
pub use crate::telegram::keyboard_button_request_user::KeyboardButtonRequestUser;
pub use crate::telegram::labeled_price::LabeledPrice;
pub use crate::telegram::location::Location;
pub use crate::telegram::login_url::LoginUrl;
pub use crate::telegram::mask_position::MaskPosition;
pub use crate::telegram::menu_button::MenuButton;
pub use crate::telegram::message::Message;
pub use crate::telegram::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
pub use crate::telegram::message_entity::MessageEntity;
pub use crate::telegram::message_entity::MessageEntityType;
pub use crate::telegram::message_id::MessageId;
pub use crate::telegram::order_info::OrderInfo;
pub use crate::telegram::passport_data::PassportData;
pub use crate::telegram::passport_element_error::PassportElementError;
pub use crate::telegram::passport_file::PassportFile;
pub use crate::telegram::photo_size::PhotoSize;
pub use crate::telegram::poll::Poll;
pub use crate::telegram::poll::PollType;
pub use crate::telegram::poll_answer::PollAnswer;
pub use crate::telegram::poll_option::PollOption;
pub use crate::telegram::pre_checkout_query::PreCheckoutQuery;
pub use crate::telegram::proximity_alert_triggered::ProximityAlertTriggered;
pub use crate::telegram::reply_keyboard_markup::ReplyKeyboardMarkup;
pub use crate::telegram::reply_keyboard_remove::ReplyKeyboardRemove;
pub use crate::telegram::response_parameters::ResponseParameters;
pub use crate::telegram::sent_web_app_message::SentWebAppMessage;
pub use crate::telegram::shipping_address::ShippingAddress;
pub use crate::telegram::shipping_option::ShippingOption;
pub use crate::telegram::shipping_query::ShippingQuery;
pub use crate::telegram::sticker::Sticker;
pub use crate::telegram::sticker::StickerType;
pub use crate::telegram::sticker_set::StickerSet;
pub use crate::telegram::successful_payment::SuccessfulPayment;
pub use crate::telegram::switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;
pub use crate::telegram::update::Update;
pub use crate::telegram::update::UpdateValue;
pub use crate::telegram::user::User;
pub use crate::telegram::user_profile_photos::UserProfilePhotos;
pub use crate::telegram::user_shared::UserShared;
pub use crate::telegram::venue::Venue;
pub use crate::telegram::video::Video;
pub use crate::telegram::video_chat_ended::VideoChatEnded;
pub use crate::telegram::video_chat_participants_invited::VideoChatParticipantsInvited;
pub use crate::telegram::video_chat_scheduled::VideoChatScheduled;
pub use crate::telegram::video_chat_started::VideoChatStarted;
pub use crate::telegram::video_note::VideoNote;
pub use crate::telegram::voice::Voice;
pub use crate::telegram::web_app_data::WebAppData;
pub use crate::telegram::web_app_info::WebAppInfo;
pub use crate::telegram::webhook_info::WebhookInfo;
pub use crate::telegram::write_access_allowed::WriteAccessAllowed;
