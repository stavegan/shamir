use crate::order_info::OrderInfo;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: Box<User>,
    pub currency: String,
    pub total_amount: u64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<Box<OrderInfo>>,
}
