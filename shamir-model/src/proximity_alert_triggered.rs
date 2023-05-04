use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProximityAlertTriggered {
    traveler: Box<User>,
    watcher: Box<User>,
    distance: u64,
}
