#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A text with some entities
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FormattedText {
    /// The text
    pub text: String,
    /// Entities contained in the text. Entities can be nested, but must not mutually intersect with each other.
    /// Pre, Code and PreCode entities can't contain other entities. BlockQuote entities can't contain other BlockQuote entities. Bold, Italic, Underline, Strikethrough, and Spoiler entities can contain and can be part of any other entities. All other entities can't contain each other
    pub entities: Vec<crate::types::TextEntity>,
}
