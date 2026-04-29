#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An underlined rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextUnderline {
    /// Text
    pub text: crate::enums::RichText,
}
