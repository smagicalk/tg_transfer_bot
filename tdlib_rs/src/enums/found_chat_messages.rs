#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundChatMessages {
    /// Contains a list of messages found by a search in a given chat
    #[serde(rename(serialize = "foundChatMessages", deserialize = "foundChatMessages"))]
    FoundChatMessages(crate::types::FoundChatMessages),
}
