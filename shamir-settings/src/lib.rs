pub mod http;
pub mod telegram;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub telegram: telegram::Settings,
}
