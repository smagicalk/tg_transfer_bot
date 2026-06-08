#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The message was originally a post in a channel
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageOriginChannel {
    /// Identifier of the channel chat to which the message was originally sent
    pub chat_id: i64,
    /// Message identifier of the original message
    pub message_id: i64,
    /// Original post author signature
    pub author_signature: String,
}
