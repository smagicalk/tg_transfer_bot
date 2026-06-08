#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMessageSender {
    /// Represents a message sender, which can be used to send messages in a chat
    #[serde(rename(serialize = "chatMessageSender", deserialize = "chatMessageSender"))]
    ChatMessageSender(crate::types::ChatMessageSender),
}
