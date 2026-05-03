#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a business chat link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessChatLinkInfo {
    /// Identifier of the private chat that created the link
    pub chat_id: i64,
    /// Message draft text that must be added to the input field
    pub text: crate::types::FormattedText,
}
