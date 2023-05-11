use crate::telegram::message_entity::MessageEntity;
use crate::telegram::poll_option::PollOption;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<Box<PollOption>>,
    pub total_voter_count: u64,
    pub is_closed: bool,
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    pub poll_type: PollType,
    pub allows_multiple_answers: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<Box<MessageEntity>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<u64>,
}

impl Poll {
    pub fn from(
        id: String,
        question: String,
        options: Vec<Box<PollOption>>,
        total_voter_count: u64,
        is_closed: bool,
        is_anonymous: bool,
        poll_type: PollType,
        allows_multiple_answers: bool,
    ) -> Self {
        Self {
            id,
            question,
            options,
            total_voter_count,
            is_closed,
            is_anonymous,
            poll_type,
            allows_multiple_answers,
            correct_option_id: None,
            explanation: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PollType {
    Regular,
    Quiz,
}

impl PollType {
    pub fn regular() -> Self {
        Self::Regular
    }

    pub fn quiz() -> Self {
        Self::Quiz
    }
}
