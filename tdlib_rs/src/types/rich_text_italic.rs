#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An italicized rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextItalic {
    /// Text
    pub text: crate::enums::RichText,
}
