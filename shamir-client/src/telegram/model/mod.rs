mod copy_message_request;
mod forward_message_request;
mod reply_markup;
mod response;
mod send_message_request;
mod send_photo_request;
mod set_webhook_request;

pub use crate::telegram::model::copy_message_request::CopyMessageRequest;
pub use crate::telegram::model::forward_message_request::ForwardMessageRequest;
pub use crate::telegram::model::reply_markup::ReplyMarkup;
pub use crate::telegram::model::response::Response;
pub use crate::telegram::model::send_message_request::SendMessageRequest;
pub use crate::telegram::model::send_photo_request::SendPhotoRequest;
pub use crate::telegram::model::set_webhook_request::SetWebhookRequest;
