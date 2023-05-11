use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackGame;

impl CallbackGame {
    pub fn from() -> Self {
        Self
    }
}
