use crate::telegram::labeled_price::LabeledPrice;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<Box<LabeledPrice>>,
}
