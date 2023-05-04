use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteAccessAllowed {
    pub web_app_name: Option<String>,
}
