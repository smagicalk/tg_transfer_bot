#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The message was originally sent on behalf of a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageOriginChat {
    /// Identifier of the chat that originally sent the message
    pub sender_chat_id: i64,
    /// For messages originally sent by an anonymous chat administrator, original message author signature
    pub author_signature: String,
}
