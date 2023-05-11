use crate::telegram::web_app_info::WebAppInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButton {
    Commands,
    WebApp { text: String, web_app: WebAppInfo },
    Default,
}

impl MenuButton {
    pub fn commands() -> Self {
        Self::Commands
    }

    pub fn web_app(text: String, web_app: WebAppInfo) -> Self {
        Self::WebApp { text, web_app }
    }

    pub fn default() -> Self {
        Self::Default
    }
}
