use crate::telegram::order_info::OrderInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: u64,
    pub invoice_payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<Box<OrderInfo>>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

impl SuccessfulPayment {
    pub fn from(
        currency: String,
        total_amount: u64,
        invoice_payload: String,
        telegram_payment_charge_id: String,
        provider_payment_charge_id: String,
    ) -> Self {
        Self {
            currency,
            total_amount,
            invoice_payload,
            shipping_option_id: None,
            order_info: None,
            telegram_payment_charge_id,
            provider_payment_charge_id,
        }
    }
}
