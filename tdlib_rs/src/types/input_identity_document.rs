#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An identity document to be saved to Telegram Passport
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputIdentityDocument {
    /// Document number; 1-24 characters
    pub number: String,
    /// Document expiration date; pass null if not applicable
    pub expiration_date: Option<crate::types::Date>,
    /// Front side of the document
    pub front_side: crate::enums::InputFile,
    /// Reverse side of the document; only for driver license and identity card; pass null otherwise
    pub reverse_side: Option<crate::enums::InputFile>,
    /// Selfie with the document; pass null if unavailable
    pub selfie: Option<crate::enums::InputFile>,
    /// List of files containing a certified English translation of the document
    pub translation: Vec<crate::enums::InputFile>,
}
