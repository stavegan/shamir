use crate::telegram::passport_file::PassportFile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub encrypted_passport_element_type: EncryptedPassportElementType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<Box<PassportFile>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<Box<PassportFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<Box<PassportFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<Box<PassportFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<Box<PassportFile>>>,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EncryptedPassportElementType {
    PersonalDetails,
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    Address,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
    PhoneNumber,
    Email,
}
