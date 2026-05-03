#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The message content has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateMessageContent {
    /// Chat identifier
    pub chat_id: i64,
    /// Message identifier
    pub message_id: i64,
    /// New message content
    pub new_content: crate::enums::MessageContent,
}
