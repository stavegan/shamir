use crate::telegram::animation::Animation;
use crate::telegram::message_entity::MessageEntity;
use crate::telegram::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<Box<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<Box<MessageEntity>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<Animation>>,
}

impl Game {
    pub fn from(title: String, description: String, photo: Vec<Box<PhotoSize>>) -> Self {
        Self {
            title,
            description,
            photo,
            text: None,
            text_entities: None,
            animation: None,
        }
    }
}
