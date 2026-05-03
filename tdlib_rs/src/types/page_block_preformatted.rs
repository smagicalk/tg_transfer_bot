#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A preformatted text paragraph
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockPreformatted {
    /// Paragraph text
    pub text: crate::enums::RichText,
    /// Programming language for which the text needs to be formatted
    pub language: String,
}
