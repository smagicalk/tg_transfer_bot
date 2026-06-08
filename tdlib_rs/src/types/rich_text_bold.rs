#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A bold rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextBold {
    /// Text
    pub text: crate::enums::RichText,
}
