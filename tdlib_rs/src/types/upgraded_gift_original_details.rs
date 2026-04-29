#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes the original details about the gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftOriginalDetails {
    /// Identifier of the user or the chat that sent the gift; may be null if the gift was private
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Identifier of the user or the chat that received the gift
    pub receiver_id: crate::enums::MessageSender,
    /// Message added to the gift
    pub text: crate::types::FormattedText,
    /// Point in time (Unix timestamp) when the gift was sent
    pub date: i32,
}
