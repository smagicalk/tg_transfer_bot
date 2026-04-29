#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A marked rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextMarked {
    /// Text
    pub text: crate::enums::RichText,
}
