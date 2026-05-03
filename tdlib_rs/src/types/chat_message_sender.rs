#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a message sender, which can be used to send messages in a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatMessageSender {
    /// The message sender
    pub sender: crate::enums::MessageSender,
    /// True, if Telegram Premium is needed to use the message sender
    pub needs_premium: bool,
}
