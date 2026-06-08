#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A personal document, containing some information about a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PersonalDocument {
    /// List of files containing the pages of the document
    pub files: Vec<crate::types::DatedFile>,
    /// List of files containing a certified English translation of the document
    pub translation: Vec<crate::types::DatedFile>,
}
