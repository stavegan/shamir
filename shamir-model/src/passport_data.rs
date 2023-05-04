use crate::encrypted_credentials::EncryptedCredentials;
use crate::encrypted_passport_element::EncryptedPassportElement;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportData {
    data: Vec<Box<EncryptedPassportElement>>,
    credentials: Box<EncryptedCredentials>,
}
