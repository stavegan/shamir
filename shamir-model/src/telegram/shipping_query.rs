use crate::telegram::shipping_address::ShippingAddress;
use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingQuery {
    pub id: String,
    pub from: Box<User>,
    pub invoice_payload: String,
    pub shipping_address: Box<ShippingAddress>,
}
