#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a list of message senders, which can be used to send messages in a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatMessageSenders {
    /// List of available message senders
    pub senders: Vec<crate::types::ChatMessageSender>,
}
