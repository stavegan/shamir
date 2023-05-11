use crate::telegram::location::Location;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatLocation {
    pub location: Box<Location>,
    pub address: String,
}

impl ChatLocation {
    pub fn from(location: Box<Location>, address: String) -> Self {
        Self { location, address }
    }
}
