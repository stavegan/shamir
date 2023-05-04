use crate::animation::Animation;
use crate::message_entity::MessageEntity;
use crate::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<Box<PhotoSize>>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<Box<MessageEntity>>>,
    pub animation: Option<Box<Animation>>,
}
