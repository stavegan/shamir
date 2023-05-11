use crate::telegram::encrypted_credentials::EncryptedCredentials;
use crate::telegram::encrypted_passport_element::EncryptedPassportElement;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportData {
    pub data: Vec<Box<EncryptedPassportElement>>,
    pub credentials: Box<EncryptedCredentials>,
}

impl PassportData {
    pub fn from(
        data: Vec<Box<EncryptedPassportElement>>,
        credentials: Box<EncryptedCredentials>,
    ) -> Self {
        Self { data, credentials }
    }
}
