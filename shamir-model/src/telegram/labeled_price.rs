use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: u64,
}

impl LabeledPrice {
    pub fn from(label: String, amount: u64) -> Self {
        Self { label, amount }
    }
}
