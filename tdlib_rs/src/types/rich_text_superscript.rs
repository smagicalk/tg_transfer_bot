#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A superscript rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextSuperscript {
    /// Text
    pub text: crate::enums::RichText,
}
