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

impl PassportElementError {
    pub fn data_field(
        data_field_type: EncryptedPassportElementType,
        field_name: String,
        data_hash: String,
        message: String,
    ) -> Self {
        Self::DataField {
            data_field_type,
            field_name,
            data_hash,
            message,
        }
    }

    pub fn front_side(
        front_side_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    ) -> Self {
        Self::FrontSide {
            front_side_type,
            file_hash,
            message,
        }
    }

    pub fn reverse_side(
        reverse_side_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    ) -> Self {
        Self::ReverseSide {
            reverse_side_type,
            file_hash,
            message,
        }
    }

    pub fn selfie(
        selfie_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    ) -> Self {
        Self::Selfie {
            selfie_type,
            file_hash,
            message,
        }
    }

    pub fn file(
        file_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    ) -> Self {
        Self::File {
            file_type,
            file_hash,
            message,
        }
    }

    pub fn files(
        files_type: EncryptedPassportElementType,
        file_hashes: Vec<String>,
        message: String,
    ) -> Self {
        Self::Files {
            files_type,
            file_hashes,
            message,
        }
    }

    pub fn translation_file(
        translation_file_type: EncryptedPassportElementType,
        file_hash: String,
        message: String,
    ) -> Self {
        Self::TranslationFile {
            translation_file_type,
            file_hash,
            message,
        }
    }

    pub fn translation_files(
        translation_files_type: EncryptedPassportElementType,
        file_hashes: Vec<String>,
        message: String,
    ) -> Self {
        Self::TranslationFiles {
            translation_files_type,
            file_hashes,
            message,
        }
    }

    pub fn unspecified(
        unspecified_type: EncryptedPassportElementType,
        element_hash: String,
        message: String,
    ) -> Self {
        Self::Unspecified {
            unspecified_type,
            element_hash,
            message,
        }
    }
}
