use crate::telegram::order_info::OrderInfo;
use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: Box<User>,
    pub currency: String,
    pub total_amount: u64,
    pub invoice_payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<Box<OrderInfo>>,
}

impl PreCheckoutQuery {
    pub fn from(
        id: String,
        from: Box<User>,
        currency: String,
        total_amount: u64,
        invoice_payload: String,
    ) -> Self {
        Self {
            id,
            from,
            currency,
            total_amount,
            invoice_payload,
            shipping_option_id: None,
            order_info: None,
        }
    }
}
