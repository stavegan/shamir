use crate::http;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub token: String,
    pub http: http::Settings,
}
