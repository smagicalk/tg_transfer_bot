#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about a forwarded message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageForwardInfo {
    /// Origin of the forwarded message
    pub origin: crate::enums::MessageOrigin,
    /// Point in time (Unix timestamp) when the message was originally sent
    pub date: i32,
    /// For messages forwarded to the chat with the current user (Saved Messages), to the Replies bot chat, or to the channel's discussion group, information about the source message from which the message was forwarded last time; may be null for other forwards or if unknown
    pub source: Option<crate::types::ForwardSource>,
    /// The type of public service announcement for the forwarded message
    pub public_service_announcement_type: String,
}
