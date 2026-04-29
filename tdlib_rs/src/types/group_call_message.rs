#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a message sent in a group call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GroupCallMessage {
    /// Unique message identifier within the group call
    pub message_id: i32,
    /// Identifier of the sender of the message
    pub sender_id: crate::enums::MessageSender,
    /// Point in time (Unix timestamp) when the message was sent
    pub date: i32,
    /// Text of the message. If empty, then the message is a paid reaction in a live story
    pub text: crate::types::FormattedText,
    /// The number of Telegram Stars that were paid to send the message; for live stories only
    pub paid_message_star_count: i64,
    /// True, if the message is sent by the owner of the call and must be treated as a message of the maximum level; for live stories only
    pub is_from_owner: bool,
    /// True, if the message can be deleted by the current user; for live stories only
    pub can_be_deleted: bool,
}
