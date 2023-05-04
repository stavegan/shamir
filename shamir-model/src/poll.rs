use crate::message_entity::MessageEntity;
use crate::poll_option::PollOption;
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
    pub correct_option_id: Option<u64>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<Box<MessageEntity>>>,
    pub open_period: Option<u64>,
    pub close_date: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PollType {
    Regular,
    Quiz,
}
