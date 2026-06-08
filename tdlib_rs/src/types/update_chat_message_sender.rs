#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The message sender that is selected to send messages in a chat has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatMessageSender {
    /// Chat identifier
    pub chat_id: i64,
    /// New value of message_sender_id; may be null if the user can't change message sender
    pub message_sender_id: Option<crate::enums::MessageSender>,
}
