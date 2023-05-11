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

impl ShippingQuery {
    pub fn from(
        id: String,
        from: Box<User>,
        invoice_payload: String,
        shipping_address: Box<ShippingAddress>,
    ) -> Self {
        Self {
            id,
            from,
            invoice_payload,
            shipping_address,
        }
    }
}
