#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TargetChat {
    /// The currently opened chat and forum topic must be kept
    #[serde(rename(serialize = "targetChatCurrent", deserialize = "targetChatCurrent"))]
    Current,
    /// The chat needs to be chosen by the user among chats of the specified types
    #[serde(rename(serialize = "targetChatChosen", deserialize = "targetChatChosen"))]
    Chosen(crate::types::TargetChatChosen),
    /// The chat needs to be open with the provided internal link
    #[serde(rename(serialize = "targetChatInternalLink", deserialize = "targetChatInternalLink"))]
    InternalLink(Box<crate::types::TargetChatInternalLink>),
}
