#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMessageSenders {
    /// Represents a list of message senders, which can be used to send messages in a chat
    #[serde(rename(serialize = "chatMessageSenders", deserialize = "chatMessageSenders"))]
    ChatMessageSenders(crate::types::ChatMessageSenders),
}
