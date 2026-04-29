#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatAvailableReactions {
    /// All reactions are available in the chat, excluding the paid reaction and custom reactions in channel chats
    #[serde(rename(serialize = "chatAvailableReactionsAll", deserialize = "chatAvailableReactionsAll"))]
    All(crate::types::ChatAvailableReactionsAll),
    /// Only specific reactions are available in the chat
    #[serde(rename(serialize = "chatAvailableReactionsSome", deserialize = "chatAvailableReactionsSome"))]
    Some(crate::types::ChatAvailableReactionsSome),
}
