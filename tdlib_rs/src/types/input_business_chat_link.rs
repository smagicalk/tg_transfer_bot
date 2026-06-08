#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a business chat link to create or edit
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputBusinessChatLink {
    /// Message draft text that will be added to the input field
    pub text: crate::types::FormattedText,
    /// Link title
    pub title: String,
}
