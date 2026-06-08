#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A subscript rich text
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextSubscript {
    /// Text
    pub text: crate::enums::RichText,
}
