use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: u64,
}

impl Invoice {
    pub fn from(
        title: String,
        description: String,
        start_parameter: String,
        currency: String,
        total_amount: u64,
    ) -> Self {
        Self {
            title,
            description,
            start_parameter,
            currency,
            total_amount,
        }
    }
}
