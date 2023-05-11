use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

impl ShippingAddress {
    pub fn from(
        country_code: String,
        state: String,
        city: String,
        street_line1: String,
        street_line2: String,
        post_code: String,
    ) -> Self {
        Self {
            country_code,
            state,
            city,
            street_line1,
            street_line2,
            post_code,
        }
    }
}
