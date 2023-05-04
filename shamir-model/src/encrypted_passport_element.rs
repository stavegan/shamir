use crate::passport_file::PassportFile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub encrypted_passport_element_type: EncryptedPassportElementType,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<Box<PassportFile>>>,
    pub front_side: Option<Box<PassportFile>>,
    pub reverse_side: Option<Box<PassportFile>>,
    pub selfie: Option<Box<PassportFile>>,
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
