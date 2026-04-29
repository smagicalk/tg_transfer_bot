#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputMessageReplyTo {
    /// Describes a message to be replied in the same chat and forum topic
    #[serde(rename(serialize = "inputMessageReplyToMessage", deserialize = "inputMessageReplyToMessage"))]
    Message(crate::types::InputMessageReplyToMessage),
    /// Describes a message to be replied that is from a different chat or a forum topic; not supported in secret chats
    #[serde(rename(serialize = "inputMessageReplyToExternalMessage", deserialize = "inputMessageReplyToExternalMessage"))]
    ExternalMessage(crate::types::InputMessageReplyToExternalMessage),
    /// Describes a story to be replied
    #[serde(rename(serialize = "inputMessageReplyToStory", deserialize = "inputMessageReplyToStory"))]
    Story(crate::types::InputMessageReplyToStory),
}
