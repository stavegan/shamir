use crate::telegram::location::Location;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatLocation {
    pub location: Box<Location>,
    pub address: String,
}
