use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

impl Contact {
    pub fn from(phone_number: String, first_name: String) -> Self {
        Self {
            phone_number,
            first_name,
            last_name: None,
            user_id: None,
            vcard: None,
        }
    }
}
