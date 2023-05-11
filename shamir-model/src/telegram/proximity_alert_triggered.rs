use crate::telegram::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProximityAlertTriggered {
    pub traveler: Box<User>,
    pub watcher: Box<User>,
    pub distance: u64,
}

impl ProximityAlertTriggered {
    pub fn from(traveler: Box<User>, watcher: Box<User>, distance: u64) -> Self {
        Self {
            traveler,
            watcher,
            distance,
        }
    }
}
