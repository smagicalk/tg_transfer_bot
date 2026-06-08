#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A rich text email link
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RichTextEmailAddress {
    /// Text
    pub text: crate::enums::RichText,
    /// Email address
    pub email_address: String,
}
