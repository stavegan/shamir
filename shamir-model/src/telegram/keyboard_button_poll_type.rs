use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub keyboard_button_poll_type_type: Option<KeyboardButtonPollTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardButtonPollTypeType {
    Quiz,
    Regular,
}
