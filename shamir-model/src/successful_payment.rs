use crate::order_info::OrderInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: u64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<Box<OrderInfo>>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}
