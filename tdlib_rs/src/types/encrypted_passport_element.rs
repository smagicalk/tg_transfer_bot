#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about an encrypted Telegram Passport element; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EncryptedPassportElement {
    /// Type of Telegram Passport element
    pub r#type: crate::enums::PassportElementType,
    /// Encrypted JSON-encoded data about the user
    pub data: String,
    /// The front side of an identity document
    pub front_side: crate::types::DatedFile,
    /// The reverse side of an identity document; may be null
    pub reverse_side: Option<crate::types::DatedFile>,
    /// Selfie with the document; may be null
    pub selfie: Option<crate::types::DatedFile>,
    /// List of files containing a certified English translation of the document
    pub translation: Vec<crate::types::DatedFile>,
    /// List of attached files
    pub files: Vec<crate::types::DatedFile>,
    /// Unencrypted data, phone number or email address
    pub value: String,
    /// Hash of the entire element
    pub hash: String,
}
