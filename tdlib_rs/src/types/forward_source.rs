#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about the last message from which a new message was forwarded last time
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ForwardSource {
    /// Identifier of the chat to which the message that was forwarded belonged; may be 0 if unknown
    pub chat_id: i64,
    /// Identifier of the message; may be 0 if unknown
    pub message_id: i64,
    /// Identifier of the sender of the message; may be null if unknown or the new message was forwarded not to Saved Messages
    pub sender_id: Option<crate::enums::MessageSender>,
    /// Name of the sender of the message if the sender is hidden by their privacy settings
    pub sender_name: String,
    /// Point in time (Unix timestamp) when the message is sent; 0 if unknown
    pub date: i32,
    /// True, if the message that was forwarded is outgoing; always false if sender is unknown
    pub is_outgoing: bool,
}
