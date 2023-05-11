use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

impl EncryptedCredentials {
    pub fn from(data: String, hash: String, secret: String) -> Self {
        Self { data, hash, secret }
    }
}
