use crate::telegram::labeled_price::LabeledPrice;
use crate::telegram::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InputMessageContent {
    Message {
        message_text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        entities: Option<Vec<Box<MessageEntity>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_web_page_preview: Option<bool>,
    },
    Venue {
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_type: Option<String>,
    },
    Location {
        latitude: f64,
        longitude: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        horizontal_accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        live_period: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        heading: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        proximity_alert_radius: Option<u64>,
    },
    Contact {
        phone_number: String,
        first_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        vcard: Option<String>,
    },
    Invoice {
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<Box<LabeledPrice>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_tip_amount: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        suggested_tip_amounts: Option<Vec<u64>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_size: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_width: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_height: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        need_name: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        need_phone_number: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        need_email: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        need_shipping_address: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        send_phone_number_to_provider: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        send_email_to_provider: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_flexible: Option<bool>,
    },
}

impl InputMessageContent {
    pub fn message(message_text: String) -> Self {
        Self::Message {
            message_text,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
        }
    }

    pub fn venue(latitude: f64, longitude: f64, title: String, address: String) -> Self {
        Self::Venue {
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }

    pub fn location(latitude: f64, longitude: f64) -> Self {
        Self::Location {
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }

    pub fn contact(phone_number: String, first_name: String) -> Self {
        Self::Contact {
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
        }
    }

    pub fn invoice(
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<Box<LabeledPrice>>,
    ) -> Self {
        Self::Invoice {
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
        }
    }
}
