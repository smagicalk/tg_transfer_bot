#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A text paragraph
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PageBlockParagraph {
    /// Paragraph text
    pub text: crate::enums::RichText,
}
