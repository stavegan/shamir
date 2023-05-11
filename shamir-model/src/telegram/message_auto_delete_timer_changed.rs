use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: u64,
}

impl MessageAutoDeleteTimerChanged {
    pub fn from(message_auto_delete_time: u64) -> Self {
        Self {
            message_auto_delete_time,
        }
    }
}
