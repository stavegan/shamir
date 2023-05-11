use crate::telegram::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultsButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<Box<WebAppInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}
