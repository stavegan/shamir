use crate::telegram::labeled_price::LabeledPrice;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<Box<LabeledPrice>>,
}

impl ShippingOption {
    pub fn from(id: String, title: String, prices: Vec<Box<LabeledPrice>>) -> Self {
        Self { id, title, prices }
    }
}
