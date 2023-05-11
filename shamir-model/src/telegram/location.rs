use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u64>,
}

impl Location {
    pub fn from(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude,
            latitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }
}
