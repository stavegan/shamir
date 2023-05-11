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

impl EncryptedPassportElement {
    pub fn from(
        encrypted_passport_element_type: EncryptedPassportElementType,
        hash: String,
    ) -> Self {
        Self {
            encrypted_passport_element_type,
            data: None,
            phone_number: None,
            email: None,
            files: None,
            front_side: None,
            reverse_side: None,
            selfie: None,
            translation: None,
            hash,
        }
    }
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

impl EncryptedPassportElementType {
    pub fn personal_details() -> Self {
        Self::PersonalDetails
    }

    pub fn passport() -> Self {
        Self::Passport
    }

    pub fn driver_license() -> Self {
        Self::DriverLicense
    }

    pub fn identity_card() -> Self {
        Self::IdentityCard
    }

    pub fn internal_passport() -> Self {
        Self::InternalPassport
    }

    pub fn address() -> Self {
        Self::Address
    }

    pub fn utility_bill() -> Self {
        Self::UtilityBill
    }

    pub fn bank_statement() -> Self {
        Self::BankStatement
    }

    pub fn rental_agreement() -> Self {
        Self::RentalAgreement
    }

    pub fn passport_registration() -> Self {
        Self::PassportRegistration
    }

    pub fn temporary_registration() -> Self {
        Self::TemporaryRegistration
    }

    pub fn phone_number() -> Self {
        Self::PhoneNumber
    }

    pub fn email() -> Self {
        Self::Email
    }
}
