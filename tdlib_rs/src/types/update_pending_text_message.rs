#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new pending text message was received in a chat with a bot. The message must be shown in the chat for at most getOption("pending_text_message_period") seconds,
/// replace any other pending message with the same draft_id, and be deleted whenever any incoming message from the bot in the message thread is received
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdatePendingTextMessage {
    /// Chat identifier
    pub chat_id: i64,
    /// The forum topic identifier in which the message will be sent; 0 if none
    pub forum_topic_id: i32,
    /// Unique identifier of the message draft within the message thread
    #[serde_as(as = "DisplayFromStr")]
    pub draft_id: i64,
    /// Text of the pending message
    pub text: crate::types::FormattedText,
}
