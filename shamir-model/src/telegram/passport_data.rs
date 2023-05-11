use crate::telegram::encrypted_credentials::EncryptedCredentials;
use crate::telegram::encrypted_passport_element::EncryptedPassportElement;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportData {
    pub data: Vec<Box<EncryptedPassportElement>>,
    pub credentials: Box<EncryptedCredentials>,
}
