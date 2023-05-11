use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: u64,
}
