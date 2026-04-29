#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An identity document
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IdentityDocument {
    /// Document number; 1-24 characters
    pub number: String,
    /// Document expiration date; may be null if not applicable
    pub expiration_date: Option<crate::types::Date>,
    /// Front side of the document
    pub front_side: crate::types::DatedFile,
    /// Reverse side of the document; only for driver license and identity card; may be null
    pub reverse_side: Option<crate::types::DatedFile>,
    /// Selfie with the document; may be null
    pub selfie: Option<crate::types::DatedFile>,
    /// List of files containing a certified English translation of the document
    pub translation: Vec<crate::types::DatedFile>,
}
