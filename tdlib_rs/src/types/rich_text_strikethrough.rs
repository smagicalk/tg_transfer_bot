#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A strikethrough rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextStrikethrough {
    /// Text
    pub text: crate::enums::RichText,
}
