#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A personal document to be saved to Telegram Passport
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputPersonalDocument {
    /// List of files containing the pages of the document
    pub files: Vec<crate::enums::InputFile>,
    /// List of files containing a certified English translation of the document
    pub translation: Vec<crate::enums::InputFile>,
}
