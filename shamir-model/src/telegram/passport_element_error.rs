use crate::telegram::encrypted_passport_element::EncryptedPassportElementType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum PassportElementError {
    #[serde(rename = "data")]
    DataField {
        #[serde(rename = "type")]
        data_field_type: EncryptedPassportElementType,
        field_name: String,
        data_hash: String,
        message: String,
    },
    FrontSide {
        #[serde(rename = "type")]
        front_side_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    ReverseSide {
        #[serde(rename = "type")]
        reverse_side_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    Selfie {
        #[serde(rename = "type")]
        selfie_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    File {
        #[serde(rename = "type")]
        file_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    Files {
        #[serde(rename = "type")]
        files_type: EncryptedPassportElementType,
        file_hashes: Vec<String>,
        message: String,
    },
    TranslationFile {
        #[serde(rename = "type")]
        translation_file_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    },
    TranslationFiles {
        #[serde(rename = "type")]
        translation_files_type: EncryptedPassportElementType,
        file_hashes: Vec<String>,
        message: String,
    },
    Unspecified {
        #[serde(rename = "type")]
        unspecified_type: EncryptedPassportElementType,
        element_hash: String,
        message: String,
    },
}
